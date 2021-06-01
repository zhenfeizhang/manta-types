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
use ark_std::{rand::RngCore, vec::Vec};
use manta_crypto::{CommitmentParam, MantaSerDes, COMMIT_PARAM};

#[test]
fn test_manta_random_asset_serdes() {
	let mut rng = ark_std::test_rng();
	let commit_param = CommitmentParam::deserialize(COMMIT_PARAM.data).unwrap();
	let asset_id = AssetId::default();
	let value = 10;
	let mut secret_key = [0u8; 32];
	rng.fill_bytes(&mut secret_key);

	let random_asset =
		MantaAsset::sample(&commit_param, &secret_key, &asset_id, &value, &mut rng).unwrap();
	let mut buf: Vec<u8> = Vec::new();
	random_asset.serialize(&mut buf).unwrap();
	let random_asset_recover = MantaAsset::deserialize(buf.as_ref()).unwrap();
	assert_eq!(random_asset, random_asset_recover);
}

#[test]
fn test_manta_random_receiver_serdes() {
	let mut rng = ark_std::test_rng();
	let commit_param = CommitmentParam::deserialize(COMMIT_PARAM.data).unwrap();
	let asset_id = AssetId::default();
	let mut secret_key = [0u8; 32];
	rng.fill_bytes(&mut secret_key);

	let random_receiver =
		MantaAssetFullReceiver::sample(&commit_param, &secret_key, &asset_id, &(), &mut rng)
			.unwrap();
	let mut buf: Vec<u8> = Vec::new();
	random_receiver.serialize(&mut buf).unwrap();
	let random_receiver_recover = MantaAssetFullReceiver::deserialize(buf.as_ref()).unwrap();
	assert_eq!(random_receiver, random_receiver_recover);
}
