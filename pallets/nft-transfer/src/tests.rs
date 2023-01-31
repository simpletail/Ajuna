// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
	mock::*,
	traits::{AssetCode, NFTConvertible, *},
	Error, *,
};
use codec::{Decode, Encode};
use frame_support::{assert_noop, assert_ok};

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
struct MockStruct {
	data: Vec<u8>,
}

impl Default for MockStruct {
	fn default() -> Self {
		Self { data: vec![1; 32] }
	}
}

impl NFTConvertible for MockStruct {
	fn get_asset_code() -> AssetCode {
		1
	}
}

mod store_as_nft {
	use super::*;

	#[test]
	fn asset_properly_stored() {
		ExtBuilder::default().build().execute_with(|| {
			let collection_id = 1;
			let asset = MockStruct::default();

			let result = NFTTransfer::store_as_nft(BOB, ALICE, collection_id, asset.clone());

			assert_ok!(result);

			let asset_id = result.unwrap();

			System::assert_last_event(mock::RuntimeEvent::NFTTransfer(crate::Event::AssetStored {
				collection_id,
				asset_id,
				owner: BOB,
			}));

			assert_eq!(
				LockItemStatus::<Test>::get(collection_id, asset_id),
				Some(NFTStatus::Stored)
			);

			let stored_asset = AttributeStorage::<Test>::get(
				(collection_id, asset_id),
				MockStruct::get_asset_code(),
			)
			.map(|item| item.into_inner());

			assert_eq!(stored_asset, Some(asset.encode_into()))
		});
	}

	#[test]
	fn cannot_store_asset_above_encoding_limit() {
		ExtBuilder::default().build().execute_with(|| {
			let collection_id = 1;
			let asset = MockStruct { data: vec![1; MAX_ENCODING_SIZE as usize] };

			assert_noop!(
				NFTTransfer::store_as_nft(BOB, ALICE, collection_id, asset),
				Error::<Test>::AssetSizeAboveEncodingLimit
			);
		});
	}
}

mod recover_from_nft {
	use super::*;

	#[test]
	fn asset_properly_recovered() {
		ExtBuilder::default().build().execute_with(|| {
			let collection_id = 1;
			let asset = MockStruct::default();

			let asset_id = NFTTransfer::store_as_nft(BOB, ALICE, collection_id, asset.clone())
				.expect("Storage should have been successful!");

			let result = NFTTransfer::recover_from_nft(BOB, collection_id, asset_id);

			assert_eq!(result, Ok(asset));

			System::assert_last_event(mock::RuntimeEvent::NFTTransfer(
				crate::Event::AssetRestored { collection_id, asset_id, owner: BOB },
			));

			assert_eq!(LockItemStatus::<Test>::get(collection_id, asset_id), None);

			let stored_asset = AttributeStorage::<Test>::get(
				(collection_id, asset_id),
				MockStruct::get_asset_code(),
			);

			assert_eq!(stored_asset, None);
		});
	}

	#[test]
	fn cannot_restore_uploaded_asset() {
		ExtBuilder::default().build().execute_with(|| {
			let collection_id = 1;
			let asset = MockStruct::default();

			let asset_id = NFTTransfer::store_as_nft(BOB, ALICE, collection_id, asset)
				.expect("Storage should have been successful!");

			LockItemStatus::<Test>::insert(collection_id, asset_id, NFTStatus::Uploaded);

			let result: Result<MockStruct, _> =
				NFTTransfer::recover_from_nft(BOB, collection_id, asset_id);

			assert_noop!(result, Error::<Test>::NFTOutsideOfChain);
		});
	}
}
