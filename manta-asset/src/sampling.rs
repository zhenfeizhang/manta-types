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
use ark_crypto_primitives::{
	commitment::pedersen::Randomness,
	prf::{Blake2s, PRF},
	CommitmentScheme as ArkCommitmentScheme,
};
use ark_ed_on_bls12_381::Fr;
use ark_ff::UniformRand;
use ark_serialize::CanonicalSerialize;
use ark_std::{
	rand::{CryptoRng, RngCore},
	vec::Vec,
};
use manta_crypto::*;
use manta_error::MantaError;

pub trait Sampling {
	type Param;
	type SecretKey;
	type Value;
	type AssetId;

	/// sampling a random asset
	/// - param: the parameter for generating the commitment
	/// - secret_key: self-explained
	/// - asset_id: self-explained
	/// - value: the number of assets held in this token
	/// - rng: self-explained
	fn sample<R: RngCore + CryptoRng>(
		param: &Self::Param,
		secret_key: &Self::SecretKey,
		asset_id: &AssetId,
		value: &Self::Value,
		rng: &mut R,
	) -> Result<Self, MantaError>
	where
		Self: Sized;
}

impl Sampling for MantaAsset {
	type Param = CommitmentParam;
	type SecretKey = [u8; 32];
	type Value = u64;
	type AssetId = AssetId;

	/// sampling a random asset
	/// - param: the parameter for generating the commitment
	/// - secret_key: self-explained
	/// - asset_id: self-explained
	/// - value: the number of assets held in this token
	/// - rng: self-explained
	fn sample<R: RngCore + CryptoRng>(
		param: &Self::Param,
		secret_key: &Self::SecretKey,
		asset_id: &AssetId,
		value: &Self::Value,
		rng: &mut R,
	) -> Result<Self, MantaError> {
		//  sample a random rho
		let mut rho = [0u8; 32];
		rng.fill_bytes(&mut rho);

		// pk = PRF(sk, 0); which is also the address
		let pk = <Blake2s as PRF>::evaluate(secret_key, &[0u8; 32])?;

		// sn = PRF(sk, rho)
		let sn = <Blake2s as PRF>::evaluate(secret_key, &rho)?;

		// k = com(pk||rho, r)
		let buf = [pk, rho].concat();

		let r = Fr::rand(rng);
		let mut r_bytes = [0u8; 32];
		r.serialize(r_bytes.as_mut())?;
		let r = Randomness(r);

		let k = CommitmentScheme::commit(&param, &buf, &r)?;
		let mut k_bytes = [0u8; 32];
		k.serialize(k_bytes.as_mut())?;

		// cm = com( asset_id | v||k, s )
		let buf: Vec<u8> = [
			(*asset_id as u64).to_le_bytes().as_ref(),
			value.to_le_bytes().as_ref(),
			k_bytes.clone().as_ref(),
		]
		.concat();

		let s = Fr::rand(rng);
		let mut s_bytes = [0u8; 32];
		s.serialize(s_bytes.as_mut())?;
		let s = Randomness(s);

		let cm = CommitmentScheme::commit(&param, &buf, &s)?;
		let mut cm_bytes = [0u8; 32];
		cm.serialize(cm_bytes.as_mut())?;

		Ok(MantaAsset {
			asset_id: *asset_id,
			utxo: cm_bytes,
			void_number: sn,
			pub_info: MantaAssetPubInfo {
				pk,
				rho,
				s: s_bytes,
				r: r_bytes,
				k: k_bytes,
			},
			priv_info: MantaAssetPrivInfo {
				value: *value,
				sk: *secret_key,
			},
		})
	}
}

impl Sampling for MantaAssetFullReceiver {
	type Param = CommitmentParam;
	type SecretKey = [u8; 32];
	type Value = ();
	type AssetId = AssetId;

	/// sampling a random asset
	/// - param: the parameter for generating the commitment
	/// - secret_key: self-explained
	/// - value: since this is a prepared receiver, we don't know the value it will hold
	/// - rng: self-explained
	fn sample<R: RngCore + CryptoRng>(
		param: &Self::Param,
		secret_key: &Self::SecretKey,
		asset_id: &AssetId,
		_value: &Self::Value,
		rng: &mut R,
	) -> Result<Self, MantaError> {
		//  sample a random rho
		let mut rho = [0u8; 32];
		rng.fill_bytes(&mut rho);

		// pk = PRF(sk, 0); which is also the address
		let pk = <Blake2s as PRF>::evaluate(secret_key, &[0u8; 32])?;

		// sn = PRF(sk, rho)
		let void_number = <Blake2s as PRF>::evaluate(secret_key, &rho)?;

		// k = com(pk||rho, r)
		let buf = [pk, rho].concat();

		let r = Fr::rand(rng);
		let mut r_bytes = [0u8; 32];
		r.serialize(r_bytes.as_mut())?;
		let r = Randomness(r);

		let k = CommitmentScheme::commit(&param, &buf, &r)?;
		let mut k_bytes = [0u8; 32];
		k.serialize(k_bytes.as_mut())?;

		let s = Fr::rand(rng);
		let mut s_bytes = [0u8; 32];
		s.serialize(s_bytes.as_mut())?;

		// sample a pair of ecies keys
		let (ecpk, ecsk) = <MantaCrypto as Ecies>::keygen(rng);

		Ok(MantaAssetFullReceiver {
			shielded_address: MantaAssetShieldedAddress {
				asset_id: *asset_id,
				k: k_bytes,
				s: s_bytes,
				r: r_bytes,
				ecpk,
			},
			spending_info: MantaAssetReceiverSpendingInfo {
				asset_id: *asset_id,
				pk,
				sk: *secret_key,
				rho,
				void_number,
				ecsk,
			},
		})
	}
}
