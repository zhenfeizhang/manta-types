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
use ark_std::vec::Vec;
use manta_crypto::{HashParam, LedgerMerkleTree};
use manta_error::MantaError;

pub trait BuildMetadata {
	type Param;
	type Ledger;
	type Metadata;

	fn build(
		&self,
		param: &Self::Param,
		ledger: &Self::Ledger,
	) -> Result<Self::Metadata, MantaError>;
}

impl BuildMetadata for MantaAsset {
	type Param = HashParam;
	type Ledger = Vec<[u8; 32]>;
	type Metadata = SenderMetaData;

	/// Build the `SenderMetaData` from sender's `MantaAsset`
	/// and the current state of the ledger.
	fn build(
		&self,
		param: &Self::Param,
		leaves: &Self::Ledger,
	) -> Result<Self::Metadata, MantaError> {
		let tree = LedgerMerkleTree::new(param.clone(), &leaves)?;
		let root = tree.root();

		let index = match leaves.iter().position(|x| *x == self.commitment) {
			Some(p) => p,
			None => {
				return Err(MantaError::LeavesNotFound);
			}
		};
		let membership = tree.generate_proof(index, &self.commitment)?;

		Ok(SenderMetaData {
			asset: self.clone(),
			root,
			membership,
		})
	}
}
