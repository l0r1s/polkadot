// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_nomination_pools`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_nomination_pools
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_nomination_pools`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_nomination_pools::WeightInfo for WeightInfo<T> {
	// Storage: NominationPools MinJoinBond (r:1 w:0)
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: System Account (r:2 w:1)
	// Storage: NominationPools MaxPoolMembersPerPool (r:1 w:0)
	// Storage: NominationPools MaxPoolMembers (r:1 w:0)
	// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: VoterList ListBags (r:2 w:2)
	fn join() -> Weight {
		// Minimum execution time: 143_368 nanoseconds.
		Weight::from_ref_time(147_813_000 as u64)
			.saturating_add(T::DbWeight::get().reads(17 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: System Account (r:3 w:2)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: VoterList ListBags (r:2 w:2)
	fn bond_extra_transfer() -> Weight {
		// Minimum execution time: 141_294 nanoseconds.
		Weight::from_ref_time(142_580_000 as u64)
			.saturating_add(T::DbWeight::get().reads(14 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:2 w:2)
	fn bond_extra_reward() -> Weight {
		// Minimum execution time: 145_976 nanoseconds.
		Weight::from_ref_time(149_745_000 as u64)
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_payout() -> Weight {
		// Minimum execution time: 54_743 nanoseconds.
		Weight::from_ref_time(55_539_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: System Account (r:2 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: VoterList ListBags (r:2 w:2)
	// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	// Storage: NominationPools CounterForSubPoolsStorage (r:1 w:1)
	fn unbond() -> Weight {
		// Minimum execution time: 143_857 nanoseconds.
		Weight::from_ref_time(145_751_000 as u64)
			.saturating_add(T::DbWeight::get().reads(18 as u64))
			.saturating_add(T::DbWeight::get().writes(13 as u64))
	}
	// Storage: NominationPools BondedPools (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn pool_withdraw_unbonded(s: u32, ) -> Weight {
		// Minimum execution time: 58_224 nanoseconds.
		Weight::from_ref_time(59_802_225 as u64)
			// Standard Error: 870
			.saturating_add(Weight::from_ref_time(19_208 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		// Minimum execution time: 98_968 nanoseconds.
		Weight::from_ref_time(100_541_263 as u64)
			// Standard Error: 1_330
			.saturating_add(Weight::from_ref_time(26_422 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	// Storage: NominationPools ReversePoolIdLookup (r:1 w:1)
	// Storage: NominationPools CounterForReversePoolIdLookup (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: NominationPools CounterForRewardPools (r:1 w:1)
	// Storage: NominationPools CounterForSubPoolsStorage (r:1 w:1)
	// Storage: NominationPools Metadata (r:1 w:1)
	// Storage: NominationPools CounterForBondedPools (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_kill(s: u32, ) -> Weight {
		// Minimum execution time: 149_053 nanoseconds.
		Weight::from_ref_time(152_258_821 as u64)
			// Standard Error: 2_471
			.saturating_add(Weight::from_ref_time(3_994 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(20 as u64))
			.saturating_add(T::DbWeight::get().writes(17 as u64))
	}
	// Storage: NominationPools LastPoolId (r:1 w:1)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: NominationPools MinCreateBond (r:1 w:0)
	// Storage: NominationPools MinJoinBond (r:1 w:0)
	// Storage: NominationPools MaxPools (r:1 w:0)
	// Storage: NominationPools CounterForBondedPools (r:1 w:1)
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools MaxPoolMembersPerPool (r:1 w:0)
	// Storage: NominationPools MaxPoolMembers (r:1 w:0)
	// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: NominationPools CounterForRewardPools (r:1 w:1)
	// Storage: NominationPools ReversePoolIdLookup (r:1 w:1)
	// Storage: NominationPools CounterForReversePoolIdLookup (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	fn create() -> Weight {
		// Minimum execution time: 131_307 nanoseconds.
		Weight::from_ref_time(131_903_000 as u64)
			.saturating_add(T::DbWeight::get().reads(21 as u64))
			.saturating_add(T::DbWeight::get().writes(15 as u64))
	}
	// Storage: NominationPools BondedPools (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: VoterList ListNodes (r:1 w:1)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	/// The range of component `n` is `[1, 16]`.
	fn nominate(n: u32, ) -> Weight {
		// Minimum execution time: 63_797 nanoseconds.
		Weight::from_ref_time(64_609_885 as u64)
			// Standard Error: 7_244
			.saturating_add(Weight::from_ref_time(987_467 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:0)
	fn set_state() -> Weight {
		// Minimum execution time: 39_822 nanoseconds.
		Weight::from_ref_time(40_523_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NominationPools BondedPools (r:1 w:0)
	// Storage: NominationPools Metadata (r:1 w:1)
	// Storage: NominationPools CounterForMetadata (r:1 w:1)
	/// The range of component `n` is `[1, 256]`.
	fn set_metadata(n: u32, ) -> Weight {
		// Minimum execution time: 15_620 nanoseconds.
		Weight::from_ref_time(16_509_178 as u64)
			// Standard Error: 137
			.saturating_add(Weight::from_ref_time(356 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: NominationPools MinJoinBond (r:0 w:1)
	// Storage: NominationPools MaxPoolMembers (r:0 w:1)
	// Storage: NominationPools MaxPoolMembersPerPool (r:0 w:1)
	// Storage: NominationPools MinCreateBond (r:0 w:1)
	// Storage: NominationPools MaxPools (r:0 w:1)
	fn set_configs() -> Weight {
		// Minimum execution time: 6_536 nanoseconds.
		Weight::from_ref_time(6_677_000 as u64)
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: NominationPools BondedPools (r:1 w:1)
	fn update_roles() -> Weight {
		// Minimum execution time: 26_150 nanoseconds.
		Weight::from_ref_time(26_682_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NominationPools BondedPools (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:1 w:1)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	fn chill() -> Weight {
		// Minimum execution time: 65_307 nanoseconds.
		Weight::from_ref_time(66_030_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: NominationPools BondedPools (r:1 w:1)
	fn set_commission() -> Weight {
		// Minimum execution time: 24_000 nanoseconds.
		Weight::from_ref_time(24_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NominationPools BondedPools (r:1 w:1)
	fn set_commission_max() -> Weight {
		// Minimum execution time: 22_000 nanoseconds.
		Weight::from_ref_time(24_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NominationPools BondedPools (r:1 w:1)
	fn set_commission_change_rate() -> Weight {
		// Minimum execution time: 21_000 nanoseconds.
		Weight::from_ref_time(23_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
