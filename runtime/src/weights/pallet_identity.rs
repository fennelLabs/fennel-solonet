
//! Autogenerated weights for pallet_identity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-17, STEPS: `1`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Seans-Mac-Mini`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/fennel-node
// benchmark
// pallet
// --chain=dev
// --wasm-execution=compiled
// --pallet=pallet_identity
// --extrinsic=*
// --steps=1
// --repeat=1
// --template=./scripts/templates/weight-template.hbs
// --output=./runtime/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_identity.
pub trait WeightInfo {
	fn create_identity() -> Weight;
	fn revoke_identity() -> Weight;
	fn revoke_identity_heavy_storage() -> Weight;
	fn add_or_update_identity_trait() -> Weight;
	fn add_or_update_long_identity_trait() -> Weight;
	fn add_or_update_many_identity_traits() -> Weight;
	fn remove_identity_trait() -> Weight;
	fn remove_identity_trait_heavy_storage() -> Weight;
	fn remove_long_identity_trait() -> Weight;
}

/// Weights for pallet_identity using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: `Identity::IdentityNumber` (r:1 w:1)
	// Proof: `Identity::IdentityNumber` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityList` (r:1 w:1)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn create_identity() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(30_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: `Identity::IdentityList` (r:1 w:1)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn revoke_identity() -> Weight {
		// Minimum execution time: 16_000 nanoseconds.
		Weight::from_ref_time(16_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:1)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn revoke_identity_heavy_storage() -> Weight {
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(29_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:0)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityTraitList` (r:1 w:1)
	// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added: 4563, mode: `MaxEncodedLen`)
	fn add_or_update_identity_trait() -> Weight {
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_ref_time(17_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:0)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityTraitList` (r:1 w:1)
	// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added: 4563, mode: `MaxEncodedLen`)
	fn add_or_update_long_identity_trait() -> Weight {
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_ref_time(18_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:0)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityTraitList` (r:1 w:1)
	// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added: 4563, mode: `MaxEncodedLen`)
	fn add_or_update_many_identity_traits() -> Weight {
		// Minimum execution time: 44_000 nanoseconds.
		Weight::from_ref_time(44_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:0)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityTraitList` (r:0 w:1)
	// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added: 4563, mode: `MaxEncodedLen`)
	fn remove_identity_trait() -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(15_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:0)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityTraitList` (r:0 w:1)
	// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added: 4563, mode: `MaxEncodedLen`)
	fn remove_identity_trait_heavy_storage() -> Weight {
		// Minimum execution time: 38_000 nanoseconds.
		Weight::from_ref_time(38_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:0)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityTraitList` (r:0 w:1)
	// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added: 4563, mode: `MaxEncodedLen`)
	fn remove_long_identity_trait() -> Weight {
		// Minimum execution time: 23_000 nanoseconds.
		Weight::from_ref_time(23_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: `Identity::IdentityNumber` (r:1 w:1)
	// Proof: `Identity::IdentityNumber` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityList` (r:1 w:1)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn create_identity() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(30_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: `Identity::IdentityList` (r:1 w:1)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn revoke_identity() -> Weight {
		// Minimum execution time: 16_000 nanoseconds.
		Weight::from_ref_time(16_000_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:1)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn revoke_identity_heavy_storage() -> Weight {
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(29_000_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:0)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityTraitList` (r:1 w:1)
	// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added: 4563, mode: `MaxEncodedLen`)
	fn add_or_update_identity_trait() -> Weight {
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_ref_time(17_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:0)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityTraitList` (r:1 w:1)
	// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added: 4563, mode: `MaxEncodedLen`)
	fn add_or_update_long_identity_trait() -> Weight {
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_ref_time(18_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:0)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityTraitList` (r:1 w:1)
	// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added: 4563, mode: `MaxEncodedLen`)
	fn add_or_update_many_identity_traits() -> Weight {
		// Minimum execution time: 44_000 nanoseconds.
		Weight::from_ref_time(44_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:0)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityTraitList` (r:0 w:1)
	// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added: 4563, mode: `MaxEncodedLen`)
	fn remove_identity_trait() -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(15_000_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:0)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityTraitList` (r:0 w:1)
	// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added: 4563, mode: `MaxEncodedLen`)
	fn remove_identity_trait_heavy_storage() -> Weight {
		// Minimum execution time: 38_000 nanoseconds.
		Weight::from_ref_time(38_000_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: `Identity::IdentityList` (r:1 w:0)
	// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityTraitList` (r:0 w:1)
	// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added: 4563, mode: `MaxEncodedLen`)
	fn remove_long_identity_trait() -> Weight {
		// Minimum execution time: 23_000 nanoseconds.
		Weight::from_ref_time(23_000_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}
