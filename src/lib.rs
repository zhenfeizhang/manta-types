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
#![no_std]

mod default;
mod metadata;
mod sanity;
mod serdes;
mod zkp;

use manta_asset::{AssetId, MantaAsset};
use manta_crypto::{AccountMembership, LedgerMerkleTreeRoot};

pub use metadata::BuildMetadata;

pub const MINT_PAYLOAD_SIZE: usize = 112;
pub const PRIVATE_TRANSFER_PAYLOAD_SIZE: usize = 608;
pub const RECLAIM_PAYLOAD_SIZE: usize = 512;

/// Type aliases
pub type MintPayload = [u8; MINT_PAYLOAD_SIZE];
pub type PrivateTransferPayload = [u8; PRIVATE_TRANSFER_PAYLOAD_SIZE];
pub type ReclaimPayload = [u8; RECLAIM_PAYLOAD_SIZE];

/// Input data to a mint extrinsic.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct MintData {
	pub asset_id: AssetId,
	pub amount: u64,
	pub cm: [u8; 32],
	pub k: [u8; 32],
	pub s: [u8; 32],
}

/// Input data to a private transfer extrinsic.
#[derive(Debug, Clone, PartialEq)]
pub struct PrivateTransferData {
	pub sender_1: SenderData,
	pub sender_2: SenderData,
	pub receiver_1: ReceiverData,
	pub receiver_2: ReceiverData,
	pub proof: [u8; 192],
}

/// Input data to a reclaim extrinsic.
#[derive(Debug, Clone, PartialEq)]
pub struct ReclaimData {
	pub asset_id: AssetId,
	pub reclaim_amount: u64,
	pub sender_1: SenderData,
	pub sender_2: SenderData,
	pub receiver: ReceiverData,
	pub proof: [u8; 192],
}

/// Data required for a sender to spend a coin.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct SenderData {
	pub k: [u8; 32],
	pub void_number: [u8; 32],
	pub root: [u8; 32],
}

/// Data required for a receiver to receive a coin.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct ReceiverData {
	pub k: [u8; 32],
	pub cm: [u8; 32],
	pub sender_pk: [u8; 32],
	pub cipher: [u8; 16],
}

/// A `SenderMetaData` is the data that a sender assembles from its `MantaAsset`
/// and the current state of the ledger. This struct is an input to both
/// `private transfer` and `reclaim` circuit.
#[derive(Debug, Clone, Default)]
pub struct SenderMetaData {
	pub asset: MantaAsset,
	pub root: LedgerMerkleTreeRoot,
	pub membership: AccountMembership,
}
