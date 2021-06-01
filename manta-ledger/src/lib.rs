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

//! This module implements the ledger for manta's private asset.
//! The private asset ledger consist of a fixed number of __256__ merkle trees.
//! Each tree is a `Shard`, and collectively they form the `Shards`.
//! When an UTXO is posted to the ledger, it will be send to the corresponding
//! shard via some deterministic fashion.

use ark_std::vec::Vec;
use codec::{Decode, Encode};
use manta_crypto::*;
use manta_error::MantaError;
use ark_std::convert::TryInto;

/// A shard is a list of commitment, and a merkle root of this list.
#[derive(Encode, Debug, Decode, Clone, Default, PartialEq)]
pub struct Shard {
	pub list: Vec<[u8; 32]>,
	pub root: [u8; 32],
}

/// A Shards is a list of Shard-s.
#[derive(Encode, Debug, Decode, Clone, PartialEq)]
pub struct MantaPrivateAssetLedger {
	pub shard: [Shard; 256],
}

pub trait LedgerSharding {
	type Commitment;
	type Root;
	type Param;

	/// root exists in the current shards
	fn check_root(&self, target: &Self::Root) -> bool;

	/// the commitment exists in the current shards
	fn exist(&self, target: &Self::Commitment) -> bool;

	/// update the shards with a new commitment
	fn update(&mut self, target: &Self::Commitment, param: Self::Param) -> Result<(), MantaError>;
}

impl LedgerSharding for MantaPrivateAssetLedger {
	type Commitment = [u8; 32];
	type Root = [u8; 32];
	type Param = HashParam;

	// root exists in the current shards
	fn check_root(&self, target: &Self::Root) -> bool {
		self.shard.iter().any(|s| s.root == *target)
	}

	fn exist(&self, target: &Self::Commitment) -> bool {
		// FIXME: at the moment, the index of the shard is determined by the first
		// byte of the cm. this may be potentially risky, since the commitment
		// is a group element, and the first byte may not be uniformly distributed
		// between 0 and 255.
		let shard_index = target[0] as usize;

		self.shard[shard_index].list.iter().any(|e| e == target)
	}

	// this function updates the ledger shards,
	// this function does not check if target already exists in the list or not
	fn update(&mut self, target: &Self::Commitment, param: Self::Param) -> Result<(), MantaError> {
		// FIXME: at the moment, the index of the shard is determined by the first
		// byte of the cm. this may be potentially risky, since the commitment
		// is a group element, and the first byte may not be uniformly distributed
		// between 0 and 255.
		let shard_index = target[0] as usize;

		// update the list, and the root accordingly
		self.shard[shard_index].list.push(*target);
		self.shard[shard_index].root =
			<MantaCrypto as MerkleTree>::root(param, &self.shard[shard_index].list)?;
		Ok(())
	}
}

impl Default for MantaPrivateAssetLedger {
	fn default() -> Self {
		
		let shard = ark_std::vec![Shard::default(); 256];

		Self {
			shard: shard
				.try_into()
				.unwrap_or_else(|_| panic!("The length is 256; qed")),
		}
	}
}
