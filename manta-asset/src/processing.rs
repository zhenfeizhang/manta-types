// Copyright 2019-2021 Manta Network.
// This file is part of manta-types.
//
// manta-types is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// manta-types is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with manta-types.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
use ark_std::{
	rand::{CryptoRng, RngCore},
	vec::Vec,
};
use manta_crypto::*;
use manta_error::MantaError;

pub trait Process {
	type Output;
	type Value;
	type Param;

	// process a prepared information to a processed information
	fn process<R: RngCore + CryptoRng>(
		&self,
		value: &Self::Value,
		rng: &mut R,
	) -> Result<Self::Output, MantaError>;
}

impl Process for MantaAssetShieldedAddress {
	type Param = CommitmentParam;
	type Output = MantaAssetProcessedReceiver;
	type Value = u64;

	// process a prepared information to a processed information
	fn process<R: RngCore + CryptoRng>(
		&self,
		value: &Self::Value,
		rng: &mut R,
	) -> Result<Self::Output, MantaError> {
		let commit_param = CommitmentParam::deserialize(COMMIT_PARAM.data)?;
		// cm = com(v||k, s)
		let buf: Vec<u8> = [value.to_le_bytes().as_ref(), self.k.as_ref()].concat();
		let commitment = <MantaCrypto as Commitment>::commit(&commit_param, &buf, &self.s)?;

		let cipher = <MantaCrypto as Ecies>::encrypt(&self.ecpk, value, rng);
		let mut ciphertext = [0u8; 16];
		ciphertext.copy_from_slice(cipher[0..16].as_ref());
		let mut sender_pk = [0u8; 32];
		sender_pk.copy_from_slice(cipher[16..48].as_ref());

		Ok(MantaAssetProcessedReceiver {
			utxo: commitment,
			value: *value,
			sender_pk,
			ciphertext,
			prepared_data: *self,
		})
	}
}
