// Copyright 2024 Moonbeam foundation
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_moonbeam_orbiters`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-09-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("moonbase-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_moonbeam_orbiters
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./benchmarking/frame-weight-template.hbs
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for `pallet_moonbeam_orbiters`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_moonbeam_orbiters::WeightInfo for WeightInfo<T> {
	/// Storage: `MoonbeamOrbiters::CollatorsPool` (r:1 w:1)
	/// Proof: `MoonbeamOrbiters::CollatorsPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Reserves` (r:1 w:0)
	/// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(1037), added: 3512, mode: `MaxEncodedLen`)
	fn collator_add_orbiter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `562`
		//  Estimated: `4502`
		// Minimum execution time: 19_808_000 picoseconds.
		Weight::from_parts(20_238_000, 4502)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `MoonbeamOrbiters::CollatorsPool` (r:1 w:1)
	/// Proof: `MoonbeamOrbiters::CollatorsPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn collator_remove_orbiter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366`
		//  Estimated: `3831`
		// Minimum execution time: 15_412_000 picoseconds.
		Weight::from_parts(16_392_000, 3831)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `MoonbeamOrbiters::CollatorsPool` (r:1 w:1)
	/// Proof: `MoonbeamOrbiters::CollatorsPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn orbiter_leave_collator_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366`
		//  Estimated: `3831`
		// Minimum execution time: 15_807_000 picoseconds.
		Weight::from_parts(16_494_000, 3831)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `MoonbeamOrbiters::MinOrbiterDeposit` (r:1 w:0)
	/// Proof: `MoonbeamOrbiters::MinOrbiterDeposit` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Reserves` (r:1 w:1)
	/// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(1037), added: 3512, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `MoonbeamOrbiters::RegisteredOrbiter` (r:0 w:1)
	/// Proof: `MoonbeamOrbiters::RegisteredOrbiter` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn orbiter_register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `4502`
		// Minimum execution time: 28_936_000 picoseconds.
		Weight::from_parts(29_752_000, 4502)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `MoonbeamOrbiters::CounterForCollatorsPool` (r:1 w:0)
	/// Proof: `MoonbeamOrbiters::CounterForCollatorsPool` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `MoonbeamOrbiters::CollatorsPool` (r:101 w:0)
	/// Proof: `MoonbeamOrbiters::CollatorsPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Reserves` (r:1 w:1)
	/// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(1037), added: 3512, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `MoonbeamOrbiters::RegisteredOrbiter` (r:0 w:1)
	/// Proof: `MoonbeamOrbiters::RegisteredOrbiter` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[0, 100]`.
	fn orbiter_unregister(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `391 + n * (48 ±0)`
		//  Estimated: `4502 + n * (2524 ±0)`
		// Minimum execution time: 36_235_000 picoseconds.
		Weight::from_parts(36_686_015, 4502)
			// Standard Error: 8_456
			.saturating_add(Weight::from_parts(7_100_126, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 2524).saturating_mul(n.into()))
	}
	/// Storage: `MoonbeamOrbiters::CollatorsPool` (r:1 w:1)
	/// Proof: `MoonbeamOrbiters::CollatorsPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MoonbeamOrbiters::CounterForCollatorsPool` (r:1 w:1)
	/// Proof: `MoonbeamOrbiters::CounterForCollatorsPool` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn add_collator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `82`
		//  Estimated: `3547`
		// Minimum execution time: 10_237_000 picoseconds.
		Weight::from_parts(10_886_000, 3547)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `MoonbeamOrbiters::CollatorsPool` (r:1 w:1)
	/// Proof: `MoonbeamOrbiters::CollatorsPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MoonbeamOrbiters::CounterForCollatorsPool` (r:1 w:1)
	/// Proof: `MoonbeamOrbiters::CounterForCollatorsPool` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `MoonbeamOrbiters::AccountLookupOverride` (r:0 w:9)
	/// Proof: `MoonbeamOrbiters::AccountLookupOverride` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_collator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366`
		//  Estimated: `3831`
		// Minimum execution time: 22_288_000 picoseconds.
		Weight::from_parts(23_569_000, 3831)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(11_u64))
	}
	/// Storage: `MoonbeamOrbiters::CurrentRound` (r:1 w:0)
	/// Proof: `MoonbeamOrbiters::CurrentRound` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `MoonbeamOrbiters::OrbiterPerRound` (r:100 w:100)
	/// Proof: `MoonbeamOrbiters::OrbiterPerRound` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[0, 100]`.
	fn on_initialize(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140 + x * (61 ±0)`
		//  Estimated: `1624 + x * (2537 ±0)`
		// Minimum execution time: 6_275_000 picoseconds.
		Weight::from_parts(6_444_708, 1624)
			// Standard Error: 1_217
			.saturating_add(Weight::from_parts(861_814, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2537).saturating_mul(x.into()))
	}
	/// Storage: `MoonbeamOrbiters::OrbiterPerRound` (r:1 w:1)
	/// Proof: `MoonbeamOrbiters::OrbiterPerRound` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn distribute_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `298`
		//  Estimated: `3763`
		// Minimum execution time: 20_451_000 picoseconds.
		Weight::from_parts(20_995_000, 3763)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `MoonbeamOrbiters::ForceRotation` (r:1 w:1)
	/// Proof: `MoonbeamOrbiters::ForceRotation` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `MoonbeamOrbiters::CollatorsPool` (r:2 w:1)
	/// Proof: `MoonbeamOrbiters::CollatorsPool` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MoonbeamOrbiters::OrbiterPerRound` (r:0 w:3)
	/// Proof: `MoonbeamOrbiters::OrbiterPerRound` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MoonbeamOrbiters::AccountLookupOverride` (r:0 w:3)
	/// Proof: `MoonbeamOrbiters::AccountLookupOverride` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MoonbeamOrbiters::CurrentRound` (r:0 w:1)
	/// Proof: `MoonbeamOrbiters::CurrentRound` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_new_round() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `6196`
		// Minimum execution time: 28_935_000 picoseconds.
		Weight::from_parts(29_646_000, 6196)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
}
