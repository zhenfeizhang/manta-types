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

use crate::*;
use ark_crypto_primitives::prf::{Blake2s, PRF};
use ark_std::vec::Vec;
use manta_crypto::*;
use manta_error::MantaError;

pub trait SanityCheck {
	type Param;

	/// Check if the struct is well-formed.
	/// This function is usually useful after deserialization.
	fn sanity(&self, param: &Self::Param) -> Result<bool, MantaError>;
}

impl SanityCheck for MantaAsset {
	type Param = CommitmentParam;

	fn sanity(&self, param: &Self::Param) -> Result<bool, MantaError> {
		// pk = PRF(sk, 0); which is also the address
		if self.pub_info.pk != <Blake2s as PRF>::evaluate(&self.priv_info.sk, &[0u8; 32])? {
			return Ok(false);
		}

		// sn = PRF(sk, rho)
		if self.void_number != <Blake2s as PRF>::evaluate(&self.priv_info.sk, &self.pub_info.rho)? {
			return Ok(false);
		}

		// k = com(pk||rho, r)
		let buf = [self.pub_info.pk, self.pub_info.rho].concat();
		if self.pub_info.k != <MantaCrypto as Commitment>::commit(&param, &buf, &self.pub_info.r)? {
			return Ok(false);
		}

		// cm = com( asset_id | v||k, s )
		let buf: Vec<u8> = [
			(self.asset_id as u64).to_le_bytes().as_ref(),
			self.priv_info.value.to_le_bytes().as_ref(),
			self.pub_info.k.clone().as_ref(),
		]
		.concat();
		if self.utxo != <MantaCrypto as Commitment>::commit(&param, &buf, &self.pub_info.s)? {
			return Ok(false);
		}

		Ok(true)
	}
}

impl SanityCheck for MantaAssetFullReceiver {
	type Param = CommitmentParam;

	fn sanity(&self, param: &Self::Param) -> Result<bool, MantaError> {
		// pk = PRF(sk, 0); which is also the address
		if self.spending_info.pk != <Blake2s as PRF>::evaluate(&self.spending_info.sk, &[0u8; 32])?
		{
			return Ok(false);
		}

		// sn = PRF(sk, rho)
		if self.spending_info.void_number
			!= <Blake2s as PRF>::evaluate(&self.spending_info.sk, &self.spending_info.rho)?
		{
			return Ok(false);
		}

		// k = com(pk||rho, r)
		let buf = [self.spending_info.pk, self.spending_info.rho].concat();
		if self.shielded_address.k
			!= <MantaCrypto as Commitment>::commit(&param, &buf, &self.shielded_address.r)?
		{
			return Ok(false);
		}

		Ok(true)
	}
}
