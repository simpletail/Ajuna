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

//! Autogenerated weights for pallet_ajuna_nft_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `weight-calculation-didac`, CPU: `DO-Regular`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/bajun-para
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-ajuna-nft-staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./.maintain/frame-weight-template.hbs
// --output=./pallets/ajuna-nft-staking/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_ajuna_nft_staking.
pub trait WeightInfo {
	fn set_creator() -> Weight;
	fn set_contract_collection_id() -> Weight;
	fn set_global_config() -> Weight;
	fn fund_treasury() -> Weight;
	fn create_token_reward() -> Weight;
	fn create_nft_reward() -> Weight;
	fn accept() -> Weight;
	fn claim_token_reward() -> Weight;
	fn claim_nft_reward() -> Weight;
	fn remove_token_reward() -> Weight;
	fn remove_nft_reward() -> Weight;
	fn cancel_token_reward() -> Weight;
	fn cancel_nft_reward() -> Weight;
	fn snipe_token_reward() -> Weight;
	fn snipe_nft_reward() -> Weight;
}

/// Weights for pallet_ajuna_nft_staking using the Substrate node and recommended hardware.
pub struct AjunaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AjunaWeight<T> {
	// Storage: NftStake Creator (r:0 w:1)
	fn set_creator() -> Weight {
		// Minimum execution time: 26_582 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NftStake Creator (r:1 w:0)
	// Storage: Nft Collection (r:1 w:0)
	// Storage: NftStake ContractCollectionId (r:0 w:1)
	fn set_contract_collection_id() -> Weight {
		// Minimum execution time: 84_672 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NftStake Creator (r:1 w:0)
	// Storage: NftStake LockedState (r:0 w:1)
	fn set_global_config() -> Weight {
		// Minimum execution time: 32_015 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NftStake TreasuryAccount (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn fund_treasury() -> Weight {
		// Minimum execution time: 71_758 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NftStake LockedState (r:1 w:0)
	// Storage: NftStake TreasuryAccount (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: NftStake ContractCollectionId (r:1 w:0)
	// Storage: Nft Item (r:1 w:1)
	// Storage: Nft Collection (r:1 w:1)
	// Storage: Nft CollectionConfigOf (r:1 w:0)
	// Storage: Nft ItemConfigOf (r:1 w:1)
	// Storage: NftStake Contracts (r:0 w:1)
	// Storage: Nft Account (r:0 w:1)
	fn create_token_reward() -> Weight {
		// Minimum execution time: 139_906 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: NftStake LockedState (r:1 w:0)
	// Storage: Nft Item (r:2 w:2)
	// Storage: NftStake ContractCollectionId (r:1 w:0)
	// Storage: NftStake TreasuryAccount (r:1 w:0)
	// Storage: Nft Collection (r:2 w:1)
	// Storage: Nft CollectionConfigOf (r:2 w:0)
	// Storage: Nft ItemConfigOf (r:2 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: NftStake Contracts (r:0 w:1)
	// Storage: Nft Account (r:0 w:3)
	// Storage: Nft ItemPriceOf (r:0 w:1)
	// Storage: Nft PendingSwapOf (r:0 w:1)
	fn create_nft_reward() -> Weight {
		// Minimum execution time: 178_955 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
	}
	// Storage: NftStake LockedState (r:1 w:0)
	// Storage: NftStake ContractOwners (r:1 w:1)
	// Storage: NftStake Contracts (r:1 w:0)
	// Storage: Nft Attribute (r:10 w:0)
	// Storage: NftStake TreasuryAccount (r:1 w:0)
	// Storage: Nft Item (r:11 w:11)
	// Storage: Nft Collection (r:2 w:0)
	// Storage: Nft CollectionConfigOf (r:2 w:0)
	// Storage: Nft ItemConfigOf (r:11 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: NftStake ContractCollectionId (r:1 w:0)
	// Storage: NftStake ContractEnds (r:0 w:1)
	// Storage: NftStake ContractStakedItems (r:0 w:1)
	// Storage: Nft Account (r:0 w:22)
	// Storage: Nft ItemPriceOf (r:0 w:11)
	// Storage: Nft PendingSwapOf (r:0 w:11)
	fn accept() -> Weight {
		// Minimum execution time: 789_389 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(T::DbWeight::get().reads(42 as u64))
			.saturating_add(T::DbWeight::get().writes(59 as u64))
	}
	// Storage: NftStake LockedState (r:1 w:0)
	// Storage: NftStake ContractOwners (r:1 w:1)
	// Storage: NftStake ContractEnds (r:1 w:1)
	// Storage: NftStake ContractStakedItems (r:1 w:1)
	// Storage: NftStake TreasuryAccount (r:1 w:0)
	// Storage: Nft Collection (r:2 w:1)
	// Storage: Nft CollectionConfigOf (r:1 w:0)
	// Storage: Nft ItemConfigOf (r:11 w:1)
	// Storage: Nft Item (r:11 w:11)
	// Storage: NftStake Contracts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: NftStake ContractCollectionId (r:1 w:0)
	// Storage: Nft Account (r:0 w:21)
	// Storage: Nft ItemPriceOf (r:0 w:11)
	// Storage: Nft ItemAttributesApprovalsOf (r:0 w:1)
	// Storage: Nft PendingSwapOf (r:0 w:11)
	fn claim_token_reward() -> Weight {
		// Minimum execution time: 544_620 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(T::DbWeight::get().reads(33 as u64))
			.saturating_add(T::DbWeight::get().writes(62 as u64))
	}
	// Storage: NftStake LockedState (r:1 w:0)
	// Storage: NftStake ContractOwners (r:1 w:1)
	// Storage: NftStake ContractEnds (r:1 w:1)
	// Storage: NftStake ContractStakedItems (r:1 w:1)
	// Storage: NftStake TreasuryAccount (r:1 w:0)
	// Storage: Nft Collection (r:3 w:1)
	// Storage: Nft CollectionConfigOf (r:2 w:0)
	// Storage: Nft ItemConfigOf (r:12 w:1)
	// Storage: Nft Item (r:12 w:12)
	// Storage: NftStake Contracts (r:1 w:1)
	// Storage: NftStake ContractCollectionId (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Nft Account (r:0 w:23)
	// Storage: Nft ItemPriceOf (r:0 w:12)
	// Storage: Nft ItemAttributesApprovalsOf (r:0 w:1)
	// Storage: Nft PendingSwapOf (r:0 w:12)
	fn claim_nft_reward() -> Weight {
		// Minimum execution time: 565_593 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(T::DbWeight::get().reads(37 as u64))
			.saturating_add(T::DbWeight::get().writes(67 as u64))
	}
	fn remove_token_reward() -> Weight {
		Default::default()
	}
	fn remove_nft_reward() -> Weight {
		Default::default()
	}
	fn cancel_token_reward() -> Weight {
		Default::default()
	}
	fn cancel_nft_reward() -> Weight {
		Default::default()
	}
	fn snipe_token_reward() -> Weight {
		Default::default()
	}
	fn snipe_nft_reward() -> Weight {
		Default::default()
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: NftStake Creator (r:0 w:1)
	fn set_creator() -> Weight {
		// Minimum execution time: 26_582 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: NftStake Creator (r:1 w:0)
	// Storage: Nft Collection (r:1 w:0)
	// Storage: NftStake ContractCollectionId (r:0 w:1)
	fn set_contract_collection_id() -> Weight {
		// Minimum execution time: 84_672 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: NftStake Creator (r:1 w:0)
	// Storage: NftStake LockedState (r:0 w:1)
	fn set_global_config() -> Weight {
		// Minimum execution time: 32_015 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: NftStake TreasuryAccount (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn fund_treasury() -> Weight {
		// Minimum execution time: 71_758 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: NftStake LockedState (r:1 w:0)
	// Storage: NftStake TreasuryAccount (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: NftStake ContractCollectionId (r:1 w:0)
	// Storage: Nft Item (r:1 w:1)
	// Storage: Nft Collection (r:1 w:1)
	// Storage: Nft CollectionConfigOf (r:1 w:0)
	// Storage: Nft ItemConfigOf (r:1 w:1)
	// Storage: NftStake Contracts (r:0 w:1)
	// Storage: Nft Account (r:0 w:1)
	fn create_token_reward() -> Weight {
		// Minimum execution time: 139_906 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(RocksDbWeight::get().reads(9 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	// Storage: NftStake LockedState (r:1 w:0)
	// Storage: Nft Item (r:2 w:2)
	// Storage: NftStake ContractCollectionId (r:1 w:0)
	// Storage: NftStake TreasuryAccount (r:1 w:0)
	// Storage: Nft Collection (r:2 w:1)
	// Storage: Nft CollectionConfigOf (r:2 w:0)
	// Storage: Nft ItemConfigOf (r:2 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: NftStake Contracts (r:0 w:1)
	// Storage: Nft Account (r:0 w:3)
	// Storage: Nft ItemPriceOf (r:0 w:1)
	// Storage: Nft PendingSwapOf (r:0 w:1)
	fn create_nft_reward() -> Weight {
		// Minimum execution time: 178_955 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(RocksDbWeight::get().reads(13 as u64))
			.saturating_add(RocksDbWeight::get().writes(12 as u64))
	}
	// Storage: NftStake LockedState (r:1 w:0)
	// Storage: NftStake ContractOwners (r:1 w:1)
	// Storage: NftStake Contracts (r:1 w:0)
	// Storage: Nft Attribute (r:10 w:0)
	// Storage: NftStake TreasuryAccount (r:1 w:0)
	// Storage: Nft Item (r:11 w:11)
	// Storage: Nft Collection (r:2 w:0)
	// Storage: Nft CollectionConfigOf (r:2 w:0)
	// Storage: Nft ItemConfigOf (r:11 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: NftStake ContractCollectionId (r:1 w:0)
	// Storage: NftStake ContractEnds (r:0 w:1)
	// Storage: NftStake ContractStakedItems (r:0 w:1)
	// Storage: Nft Account (r:0 w:22)
	// Storage: Nft ItemPriceOf (r:0 w:11)
	// Storage: Nft PendingSwapOf (r:0 w:11)
	fn accept() -> Weight {
		// Minimum execution time: 789_389 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(RocksDbWeight::get().reads(42 as u64))
			.saturating_add(RocksDbWeight::get().writes(59 as u64))
	}
	// Storage: NftStake LockedState (r:1 w:0)
	// Storage: NftStake ContractOwners (r:1 w:1)
	// Storage: NftStake ContractEnds (r:1 w:1)
	// Storage: NftStake ContractStakedItems (r:1 w:1)
	// Storage: NftStake TreasuryAccount (r:1 w:0)
	// Storage: Nft Collection (r:2 w:1)
	// Storage: Nft CollectionConfigOf (r:1 w:0)
	// Storage: Nft ItemConfigOf (r:11 w:1)
	// Storage: Nft Item (r:11 w:11)
	// Storage: NftStake Contracts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: NftStake ContractCollectionId (r:1 w:0)
	// Storage: Nft Account (r:0 w:21)
	// Storage: Nft ItemPriceOf (r:0 w:11)
	// Storage: Nft ItemAttributesApprovalsOf (r:0 w:1)
	// Storage: Nft PendingSwapOf (r:0 w:11)
	fn claim_token_reward() -> Weight {
		// Minimum execution time: 544_620 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(RocksDbWeight::get().reads(33 as u64))
			.saturating_add(RocksDbWeight::get().writes(62 as u64))
	}
	// Storage: NftStake LockedState (r:1 w:0)
	// Storage: NftStake ContractOwners (r:1 w:1)
	// Storage: NftStake ContractEnds (r:1 w:1)
	// Storage: NftStake ContractStakedItems (r:1 w:1)
	// Storage: NftStake TreasuryAccount (r:1 w:0)
	// Storage: Nft Collection (r:3 w:1)
	// Storage: Nft CollectionConfigOf (r:2 w:0)
	// Storage: Nft ItemConfigOf (r:12 w:1)
	// Storage: Nft Item (r:12 w:12)
	// Storage: NftStake Contracts (r:1 w:1)
	// Storage: NftStake ContractCollectionId (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Nft Account (r:0 w:23)
	// Storage: Nft ItemPriceOf (r:0 w:12)
	// Storage: Nft ItemAttributesApprovalsOf (r:0 w:1)
	// Storage: Nft PendingSwapOf (r:0 w:12)
	fn claim_nft_reward() -> Weight {
		// Minimum execution time: 565_593 nanoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(RocksDbWeight::get().reads(37 as u64))
			.saturating_add(RocksDbWeight::get().writes(67 as u64))
	}
	fn remove_token_reward() -> Weight {
		Default::default()
	}
	fn remove_nft_reward() -> Weight {
		Default::default()
	}
	fn cancel_token_reward() -> Weight {
		Default::default()
	}
	fn cancel_nft_reward() -> Weight {
		Default::default()
	}
	fn snipe_token_reward() -> Weight {
		Default::default()
	}
	fn snipe_nft_reward() -> Weight {
		Default::default()
	}
}
