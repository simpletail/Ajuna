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

use super::*;

#[test]
fn works_with_token_reward() {
	let stake_clauses = vec![
		Clause::HasAttribute(RESERVED_COLLECTION_0, 1),
		Clause::HasAttributeWithValue(RESERVED_COLLECTION_2, 3, 4),
	];
	let fee_clauses = vec![Clause::HasAttribute(RESERVED_COLLECTION_1, 11)];
	let stake_duration = 4;
	let reward = 135;
	let cancellation_fee = 111;
	let contract = Contract::default()
		.reward(Reward::Tokens(reward))
		.stake_duration(stake_duration)
		.stake_clauses(stake_clauses.clone())
		.fee_clauses(fee_clauses.clone())
		.cancel_fee(cancellation_fee);
	let contract_id = H256::random();

	let stakes = MockMints::from(MockClauses(stake_clauses));
	let stake_addresses =
		stakes.clone().into_iter().map(|(address, _, _)| address).collect::<Vec<_>>();
	let fees = MockMints::from(MockClauses(fee_clauses));
	let fee_addresses = fees.clone().into_iter().map(|(address, _, _)| address).collect::<Vec<_>>();

	let initial_balance_alice = 222;
	let initial_balance_bob = 333;

	ExtBuilder::default()
		.set_creator(ALICE)
		.balances(vec![(ALICE, initial_balance_alice), (BOB, initial_balance_bob)])
		.create_contract_collection()
		.create_contract(contract_id, contract)
		.accept_contract(vec![(BOB, stakes)], vec![(BOB, fees)], contract_id, BOB)
		.build()
		.execute_with(|| {
			// Initial balances.
			assert_eq!(
				Balances::free_balance(ALICE),
				initial_balance_alice - reward - ItemDeposit::get()
			);
			assert_eq!(NftStake::account_balance(), reward);

			// Cancel and check.
			assert_ok!(NftStake::cancel(RuntimeOrigin::signed(BOB), contract_id));
			for NftId(collection_id, item_id) in stake_addresses {
				assert_eq!(Nft::owner(collection_id, item_id), Some(BOB));
			}
			for NftId(collection_id, item_id) in fee_addresses {
				assert_eq!(Nft::owner(collection_id, item_id), Some(ALICE));
			}
			assert_eq!(Balances::free_balance(BOB), initial_balance_bob - cancellation_fee);
			assert_eq!(NftStake::account_balance(), 0);
			assert_eq!(Balances::free_balance(ALICE), initial_balance_alice + cancellation_fee);

			let contract_collection_id = ContractCollectionId::<Test>::get().unwrap();
			assert_eq!(Nft::owner(contract_collection_id, contract_id), None);
			assert_eq!(ContractAccepted::<Test>::get(contract_id), None);
			assert_eq!(ContractStakedItems::<Test>::get(contract_id), None);
			assert_eq!(ContractIds::<Test>::get(BOB), None);

			System::assert_last_event(RuntimeEvent::NftStake(crate::Event::Cancelled {
				by: BOB,
				contract_id,
			}));
		});
}

#[test]
fn works_with_nft_reward() {
	let stake_clauses = vec![
		Clause::HasAttribute(RESERVED_COLLECTION_0, 1),
		Clause::HasAttributeWithValue(RESERVED_COLLECTION_2, 3, 4),
	];
	let fee_clauses = vec![Clause::HasAttribute(RESERVED_COLLECTION_1, 11)];
	let stake_duration = 4;
	let reward_addr = NftId(RESERVED_COLLECTION_2, H256::random());
	let cancellation_fee = 111;
	let contract = Contract::default()
		.reward(Reward::Nft(reward_addr.clone()))
		.stake_duration(stake_duration)
		.stake_clauses(stake_clauses.clone())
		.fee_clauses(fee_clauses.clone())
		.cancel_fee(cancellation_fee);
	let contract_id = H256::random();

	let stakes = MockMints::from(MockClauses(stake_clauses));
	let stake_addresses =
		stakes.clone().into_iter().map(|(address, _, _)| address).collect::<Vec<_>>();
	let fees = MockMints::from(MockClauses(fee_clauses));
	let fee_addresses = fees.clone().into_iter().map(|(address, _, _)| address).collect::<Vec<_>>();

	let initial_balance_alice = 222;
	let initial_balance_bob = 333;

	ExtBuilder::default()
		.set_creator(ALICE)
		.balances(vec![(ALICE, initial_balance_alice), (BOB, initial_balance_bob)])
		.create_contract_collection()
		.create_contract(contract_id, contract)
		.accept_contract(vec![(BOB, stakes)], vec![(BOB, fees)], contract_id, BOB)
		.build()
		.execute_with(|| {
			// Initial balances.
			assert_eq!(Balances::free_balance(ALICE), initial_balance_alice - ItemDeposit::get());
			assert_eq!(Nft::owner(reward_addr.0, reward_addr.1), Some(NftStake::account_id()));

			// Cancel and check.
			assert_ok!(NftStake::cancel(RuntimeOrigin::signed(BOB), contract_id));
			for NftId(collection_id, item_id) in stake_addresses {
				assert_eq!(Nft::owner(collection_id, item_id), Some(BOB));
			}
			for NftId(collection_id, item_id) in fee_addresses {
				assert_eq!(Nft::owner(collection_id, item_id), Some(ALICE));
			}
			assert_eq!(Nft::owner(reward_addr.0, reward_addr.1), Some(ALICE));
			assert_eq!(Balances::free_balance(BOB), initial_balance_bob - cancellation_fee);
			assert_eq!(NftStake::account_balance(), 0);
			assert_eq!(Balances::free_balance(ALICE), initial_balance_alice + cancellation_fee);

			let contract_collection_id = ContractCollectionId::<Test>::get().unwrap();
			assert_eq!(Nft::owner(contract_collection_id, contract_id), None);
			assert_eq!(ContractAccepted::<Test>::get(contract_id), None);
			assert_eq!(ContractStakedItems::<Test>::get(contract_id), None);
			assert_eq!(ContractIds::<Test>::get(BOB), None);

			System::assert_last_event(RuntimeEvent::NftStake(crate::Event::Cancelled {
				by: BOB,
				contract_id,
			}));
		});
}

#[test]
fn rejects_unsigned_calls() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			NftStake::cancel(RuntimeOrigin::none(), Default::default()),
			DispatchError::BadOrigin
		);
	});
}

#[test]
fn rejects_when_pallet_is_locked() {
	ExtBuilder::default().build().execute_with(|| {
		GlobalConfigs::<Test>::mutate(|config| config.pallet_locked = true);
		assert_noop!(
			NftStake::cancel(RuntimeOrigin::signed(ALICE), Default::default()),
			Error::<Test>::PalletLocked
		);
	});
}

#[test]
fn rejects_when_contract_is_not_owned() {
	let contract = Contract::default().reward(Reward::Tokens(1)).stake_duration(2);
	let contract_id = H256::random();
	ExtBuilder::default()
		.set_creator(ALICE)
		.create_contract_collection()
		.create_contract_with_funds(contract_id, contract)
		.build()
		.execute_with(|| {
			assert_noop!(
				NftStake::cancel(RuntimeOrigin::signed(BOB), contract_id),
				Error::<Test>::ContractOwnership
			);
		});
}

#[test]
fn rejects_when_contract_is_claimable() {
	let (stake_duration, claim_duration) = (3, 5);
	let contract_id = H256::random();

	ExtBuilder::default()
		.set_creator(ALICE)
		.create_contract_collection()
		.create_contract_with_funds(
			contract_id,
			Contract::default()
				.stake_duration(stake_duration)
				.claim_duration(claim_duration),
		)
		.accept_contract(
			vec![(BOB, Default::default())],
			vec![(BOB, Default::default())],
			contract_id,
			BOB,
		)
		.build()
		.execute_with(|| {
			let accepted_at = ContractAccepted::<Test>::get(contract_id).unwrap();

			// During claim phase.
			for n in stake_duration..=(stake_duration + claim_duration) {
				run_to_block(accepted_at + n);
				assert_noop!(
					NftStake::cancel(RuntimeOrigin::signed(BOB), contract_id),
					Error::<Test>::Claimable
				);
			}

			// Regression for happy case, before reaching claim phase.
			System::set_block_number(accepted_at + (stake_duration - 1));
			assert_ok!(NftStake::cancel(RuntimeOrigin::signed(BOB), contract_id));
		});
}

#[test]
fn rejects_when_contract_is_expired() {
	let (stake_duration, claim_duration) = (3, 5);
	let contract_id = H256::random();

	ExtBuilder::default()
		.set_creator(ALICE)
		.create_contract_collection()
		.create_contract_with_funds(
			contract_id,
			Contract::default()
				.stake_duration(stake_duration)
				.claim_duration(claim_duration),
		)
		.accept_contract(
			vec![(BOB, Default::default())],
			vec![(BOB, Default::default())],
			contract_id,
			BOB,
		)
		.build()
		.execute_with(|| {
			let accepted_at = ContractAccepted::<Test>::get(contract_id).unwrap();

			// After expiry.
			run_to_block(accepted_at + stake_duration + claim_duration + 2);
			assert_noop!(
				NftStake::cancel(RuntimeOrigin::signed(BOB), contract_id),
				Error::<Test>::Expired
			);

			// Regression for happy case, before reaching claim phase.
			System::set_block_number(accepted_at + (stake_duration - 1));
			assert_ok!(NftStake::cancel(RuntimeOrigin::signed(BOB), contract_id));
		});
}