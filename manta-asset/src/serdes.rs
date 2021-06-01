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
use crate::sanity::SanityCheck;
use ark_std::io::{Read, Write};
use manta_crypto::{CommitmentParam, MantaSerDes, COMMIT_PARAM};
use manta_error::MantaError;

impl MantaSerDes for MantaAsset {
	/// This function serialize the a manta token
	fn serialize<W: Write>(&self, mut writer: W) -> Result<(), MantaError> {
		writer.write_all(&(self.asset_id as u64).to_le_bytes())?;
		writer.write_all(&self.utxo)?;
		writer.write_all(&self.void_number)?;
		self.pub_info.serialize(&mut writer)?;
		self.priv_info.serialize(&mut writer)
	}

	/// This function deserialize a manta token
	fn deserialize<R: Read>(mut reader: R) -> Result<Self, MantaError> {
		let mut asset = MantaAsset::default();

		let mut buf = [0u8; 8];
		reader.read_exact(buf.as_mut())?;
		asset.asset_id = u64::from_le_bytes(buf);

		reader.read_exact(&mut asset.utxo)?;
		reader.read_exact(&mut asset.void_number)?;
		asset.pub_info = MantaAssetPubInfo::deserialize(&mut reader)?;
		asset.priv_info = MantaAssetPrivInfo::deserialize(&mut reader)?;

		let commit_param = CommitmentParam::deserialize(COMMIT_PARAM.data)?;
		if !asset.sanity(&commit_param)? {
			Err(MantaError::SanityCheckFail)
		} else {
			Ok(asset)
		}
	}
}

impl MantaSerDes for MantaAssetPubInfo {
	/// This function serialize the public information in a manta token
	fn serialize<W: Write>(&self, mut writer: W) -> Result<(), MantaError> {
		writer.write_all(&self.pk)?;
		writer.write_all(&self.rho)?;
		writer.write_all(&self.s)?;
		writer.write_all(&self.r)?;
		writer.write_all(&self.k).map_err(|x| x.into())
	}

	/// This function deserialize the public information in a manta token
	fn deserialize<R: Read>(mut reader: R) -> Result<Self, MantaError> {
		let mut pub_info = MantaAssetPubInfo::default();
		reader.read_exact(&mut pub_info.pk)?;
		reader.read_exact(&mut pub_info.rho)?;
		reader.read_exact(&mut pub_info.s)?;
		reader.read_exact(&mut pub_info.r)?;
		reader.read_exact(&mut pub_info.k)?;

		Ok(pub_info)
	}
}

impl MantaSerDes for MantaAssetPrivInfo {
	/// This function serialize the private information in a manta token
	fn serialize<W: Write>(&self, mut writer: W) -> Result<(), MantaError> {
		writer.write_all(&self.value.to_le_bytes())?;
		writer.write_all(&self.sk).map_err(|x| x.into())
	}

	/// This function deserialize the private information in a manta token
	fn deserialize<R: Read>(mut reader: R) -> Result<Self, MantaError> {
		let mut priv_info = MantaAssetPrivInfo::default();
		let mut value_bytes = [0u8; 8];
		reader.read_exact(&mut value_bytes)?;
		priv_info.value = u64::from_le_bytes(value_bytes);
		reader.read_exact(&mut priv_info.sk)?;

		Ok(priv_info)
	}
}

impl MantaSerDes for MantaAssetFullReceiver {
	/// Serialize a struct into a writable blob.
	fn serialize<W: Write>(&self, mut writer: W) -> Result<(), MantaError> {
		self.shielded_address.serialize(&mut writer)?;
		self.spending_info.serialize(&mut writer).map_err(|x| x)
	}
	/// Deserialize a readable data into a struct.
	fn deserialize<R: Read>(mut reader: R) -> Result<Self, MantaError> {
		let receiver = MantaAssetFullReceiver {
			shielded_address: MantaAssetShieldedAddress::deserialize(&mut reader)?,
			spending_info: MantaAssetReceiverSpendingInfo::deserialize(&mut reader)?,
		};

		let commit_param = CommitmentParam::deserialize(COMMIT_PARAM.data)?;

		if !receiver.sanity(&commit_param)? {
			Err(MantaError::SanityCheckFail)
		} else {
			Ok(receiver)
		}
	}
}

impl MantaSerDes for MantaAssetReceiverSpendingInfo {
	/// Serialize a struct into a writable blob.
	fn serialize<W: Write>(&self, mut writer: W) -> Result<(), MantaError> {
		writer.write_all(&(self.asset_id as u64).to_le_bytes())?;

		writer.write_all(&self.pk)?;
		writer.write_all(&self.sk)?;
		writer.write_all(&self.rho)?;
		writer.write_all(&self.void_number)?;
		writer.write_all(&self.ecsk).map_err(|x| x.into())
	}
	/// Deserialize a readable data into a struct.
	fn deserialize<R: Read>(mut reader: R) -> Result<Self, MantaError> {
		let mut receiver = Self::default();

		let mut buf = [0u8; 8];
		reader.read_exact(buf.as_mut())?;
		receiver.asset_id = u64::from_le_bytes(buf);

		reader.read_exact(&mut receiver.pk)?;
		reader.read_exact(&mut receiver.sk)?;
		reader.read_exact(&mut receiver.rho)?;
		reader.read_exact(&mut receiver.void_number)?;
		reader.read_exact(&mut receiver.ecsk)?;
		Ok(receiver)
	}
}

impl MantaSerDes for MantaAssetShieldedAddress {
	/// Serialize a struct into a writable blob.
	fn serialize<W: Write>(&self, mut writer: W) -> Result<(), MantaError> {
		writer.write_all(&(self.asset_id as u64).to_le_bytes())?;

		writer.write_all(&self.k)?;
		writer.write_all(&self.s)?;
		writer.write_all(&self.r)?;
		writer.write_all(&self.ecpk).map_err(|x| x.into())
	}
	/// Deserialize a readable data into a struct.
	fn deserialize<R: Read>(mut reader: R) -> Result<Self, MantaError> {
		let mut receiver = Self::default();

		let mut buf = [0u8; 8];
		reader.read_exact(buf.as_mut())?;
		receiver.asset_id = u64::from_le_bytes(buf);

		reader.read_exact(&mut receiver.k)?;
		reader.read_exact(&mut receiver.s)?;
		reader.read_exact(&mut receiver.r)?;
		reader.read_exact(&mut receiver.ecpk)?;

		Ok(receiver)
	}
}

impl MantaSerDes for MantaAssetProcessedReceiver {
	/// Serialize a struct into a writable blob.
	fn serialize<W: Write>(&self, mut writer: W) -> Result<(), MantaError> {
		writer.write_all(&self.utxo)?;
		writer.write_all(&self.value.to_le_bytes())?;
		writer.write_all(&self.sender_pk)?;
		writer.write_all(&self.ciphertext)?;
		self.prepared_data.serialize(&mut writer).map_err(|x| x)
	}
	/// Deserialize a readable data into a struct.
	fn deserialize<R: Read>(mut reader: R) -> Result<Self, MantaError> {
		let mut receiver = Self::default();
		reader.read_exact(&mut receiver.utxo)?;
		let mut value_bytes = [0u8; 8];
		reader.read_exact(&mut value_bytes)?;
		receiver.value = u64::from_le_bytes(value_bytes);
		reader.read_exact(&mut receiver.sender_pk)?;
		reader.read_exact(&mut receiver.ciphertext)?;
		receiver.prepared_data = MantaAssetShieldedAddress::deserialize(&mut reader)?;

		Ok(receiver)
	}
}
