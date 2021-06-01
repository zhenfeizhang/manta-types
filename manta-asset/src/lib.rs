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

// Ensure we're `no_std` when compiling for Wasm.
#![no_std]

mod asset_id;
mod processing;
mod sampling;
mod sanity;
mod serdes;

pub use asset_id::{AssetId, TEST_ASSET};
pub use processing::Process;
pub use sampling::Sampling;
pub use sanity::SanityCheck;
#[cfg(test)]
mod test;

/// A MantaAsset is a UTXO asset. It's unique identifier is the `commitment`.
/// The UTXO is a valid one as long as the `commitment` is posted to the ledger,
/// and the `void number` is not.
/// The UTOX is spend if the `void number` is posted to the ledger.
/// The whole Asset consist of the following fields:
/// - the identifier of this asset type
/// - a `utxo` is the sole information about a token that is recorded on chain
///   it is indeed a commitment `cm = com(v||k, s)`
/// - a void number, also called nullifier, or serial number in other contents
/// - some public information
/// - and some private information
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct MantaAsset {
	// asset id
	pub asset_id: AssetId,
	// a.k.a., the commitment
	pub utxo: [u8; 32],
	// also called nullifier, or serial number in other contents
	pub void_number: [u8; 32],
	// some additional information that can be public
	pub pub_info: MantaAssetPubInfo,
	// and those should be kept secret
	pub priv_info: MantaAssetPrivInfo,
}

/// Information related to a UTXO asset that may be revealed.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct MantaAssetPubInfo {
	pub pk: [u8; 32],
	pub rho: [u8; 32],
	pub s: [u8; 32],
	pub r: [u8; 32],
	pub k: [u8; 32],
}

/// Information related to a UTXO asset that may __not__ be revealed,
/// unless the UTXO asset is spend.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct MantaAssetPrivInfo {
	pub value: u64,
	pub sk: [u8; 32],
}

/// A MantaAssetPreparedReceiver is a pair of
/// - a shielded address: the sender may use this information
///     to build a UTXO asset for the receiver;
/// - a spending information: the receiver can use this information
///     to spend the above UTXO asset.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct MantaAssetFullReceiver {
	pub shielded_address: MantaAssetShieldedAddress,
	pub spending_info: MantaAssetReceiverSpendingInfo,
}

/// A MantaAssetReceiverSpendingInfo is the set of info that
/// are needed for the receiver to spend a UTXO from a sender
/// - the identifier of this asset type
/// - pk
/// - sk
/// - rho
/// - void_number
/// - ecsk: an ECIES secret key to recover the value
/// see paper for the above definitions
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct MantaAssetReceiverSpendingInfo {
	// asset id
	pub asset_id: AssetId,
	pub pk: [u8; 32],
	pub sk: [u8; 32],
	pub rho: [u8; 32],
	pub void_number: [u8; 32],
	pub ecsk: [u8; 32],
}

/// A MantaAssetShieldedAddress is the data that a receiver use to receive utxo
/// from a sender. The sender will be able to build a `MantaAssetProcessedReceiver`
/// with this information.
/// - the identifier of this asset type
/// - some public information k
/// - some public information s
/// - some public information r
/// - ecpk: a ECIES public key for the sender to transmit `value` privately
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct MantaAssetShieldedAddress {
	// asset id
	pub asset_id: AssetId,
	pub k: [u8; 32],
	pub s: [u8; 32],
	pub r: [u8; 32],
	pub ecpk: [u8; 32],
}

/// A MantaAssetProcessedReceiver is the data struct that is build by a sender.
/// A sender will generate upon receiving a `MantaAssetPreparedReceiver`.
/// This struct is useful to generate the actual transaction.
/// - the identifier of this asset type
/// - a `utxo` is the sole information about a token that is recorded on chain
///   it is indeed a commitment `cm = com(v||k, s)`
/// - the value v
/// - sender's public key that is used to process the receiver
/// - the ciphertext that encrypts `value`, under `dh_pk`
/// - and some preparation information that is used to derive this struct.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct MantaAssetProcessedReceiver {
	pub utxo: [u8; 32],
	pub value: u64,
	pub sender_pk: [u8; 32],
	pub ciphertext: [u8; 16],
	pub prepared_data: MantaAssetShieldedAddress,
}
