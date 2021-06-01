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

//! This module implements AssetIds.

/// An asset it is a u64.
pub type AssetId = u64;

/// The default value is 0 and is for testing only
pub const TEST_ASSET: AssetId = 0;

// use frame_support::codec::{Decode, Encode};

// /// AssetId is an enum.
// /// It is convertible from u64, which gives us more than enough asset types
// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Encode, Decode)]
// pub enum AssetId {
// 	#[codec(index = 0)]
// 	// Asset for testing only.
// 	TestAsset,
// 	// some preliminary asset ids; subject to change
// 	#[codec(index = 1)]
// 	WrappedBTC,
// 	#[codec(index = 2)]
// 	WrappedETH,
// 	#[codec(index = 3)]
// 	WrappedDOT,
// 	// unsupported.
// 	#[codec(index = 255)]
// 	Unsupported,
// }

// impl Default for AssetId {
// 	fn default() -> Self {
// 		AssetId::TestAsset
// 	}
// }

// impl From<u64> for AssetId {
// 	fn from(orig: u64) -> Self {
// 		match orig {
// 			0x0 => AssetId::TestAsset,
// 			0x1 => AssetId::WrappedBTC,
// 			0x2 => AssetId::WrappedETH,
// 			0x3 => AssetId::WrappedDOT,
// 			_ => AssetId::Unsupported,
// 		}
// 	}
// }
