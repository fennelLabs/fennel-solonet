//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-28, STEPS: `10`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("fennel-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/fennel-node
// benchmark
// pallet
// --chain=fennel-local
// --wasm-execution=compiled
// --pallet=pallet_identity
// --extrinsic=*
// --steps=10
// --repeat=100
// --template=./scripts/templates/parachain-weight-template.hbs
// --output=./runtime/fennel/src/weights

#![allow(unused_parens, unused_imports)]
#![allow(clippy::unnecessary_cast, clippy::missing_docs_in_private_items)]

use core::marker::PhantomData;
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};

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
	/// Storage: `Identity::IdentityNumber` (r:1 w:1)
	/// Proof: `Identity::IdentityNumber` (`max_values`: Some(1), `max_size`: Some(4), added: 499,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityList` (r:1 w:1)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`)
	fn create_identity() -> Weight {
		Weight::from_parts(25_000_000, 3517)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:1)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`)
	fn revoke_identity() -> Weight {
		Weight::from_parts(13_000_000, 3517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:1)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`)
	fn revoke_identity_heavy_storage() -> Weight {
		Weight::from_parts(24_000_000, 3517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:0)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityTraitList` (r:1 w:1)
	/// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added:
	/// 4563, mode: `MaxEncodedLen`)
	fn add_or_update_identity_trait() -> Weight {
		Weight::from_parts(14_000_000, 5553)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:0)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityTraitList` (r:1 w:1)
	/// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added:
	/// 4563, mode: `MaxEncodedLen`)
	fn add_or_update_long_identity_trait() -> Weight {
		Weight::from_parts(17_000_000, 5553)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:0)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityTraitList` (r:1 w:1)
	/// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added:
	/// 4563, mode: `MaxEncodedLen`)
	fn add_or_update_many_identity_traits() -> Weight {
		Weight::from_parts(47_000_000, 5553)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:0)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityTraitList` (r:0 w:1)
	/// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added:
	/// 4563, mode: `MaxEncodedLen`)
	fn remove_identity_trait() -> Weight {
		Weight::from_parts(14_000_000, 3517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:0)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityTraitList` (r:0 w:1)
	/// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added:
	/// 4563, mode: `MaxEncodedLen`)
	fn remove_identity_trait_heavy_storage() -> Weight {
		Weight::from_parts(41_000_000, 3517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:0)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityTraitList` (r:0 w:1)
	/// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added:
	/// 4563, mode: `MaxEncodedLen`)
	fn remove_long_identity_trait() -> Weight {
		Weight::from_parts(16_000_000, 3517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Identity::IdentityNumber` (r:1 w:1)
	/// Proof: `Identity::IdentityNumber` (`max_values`: Some(1), `max_size`: Some(4), added: 499,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityList` (r:1 w:1)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`)
	fn create_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1190`
		//  Estimated: `3517`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(25_000_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:1)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`)
	fn revoke_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `108`
		//  Estimated: `3517`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:1)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`)
	fn revoke_identity_heavy_storage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1150`
		//  Estimated: `3517`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(24_000_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:0)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityTraitList` (r:1 w:1)
	/// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added:
	/// 4563, mode: `MaxEncodedLen`)
	fn add_or_update_identity_trait() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `108`
		//  Estimated: `5553`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 5553)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:0)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityTraitList` (r:1 w:1)
	/// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added:
	/// 4563, mode: `MaxEncodedLen`)
	fn add_or_update_long_identity_trait() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `108`
		//  Estimated: `5553`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(17_000_000, 5553)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:0)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityTraitList` (r:1 w:1)
	/// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added:
	/// 4563, mode: `MaxEncodedLen`)
	fn add_or_update_many_identity_traits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `715`
		//  Estimated: `5553`
		// Minimum execution time: 42_000_000 picoseconds.
		Weight::from_parts(47_000_000, 5553)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:0)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityTraitList` (r:0 w:1)
	/// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added:
	/// 4563, mode: `MaxEncodedLen`)
	fn remove_identity_trait() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `3517`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:0)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityTraitList` (r:0 w:1)
	/// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added:
	/// 4563, mode: `MaxEncodedLen`)
	fn remove_identity_trait_heavy_storage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `3517`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(41_000_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityList` (r:1 w:0)
	/// Proof: `Identity::IdentityList` (`max_values`: None, `max_size`: Some(52), added: 2527,
	/// mode: `MaxEncodedLen`) Storage: `Identity::IdentityTraitList` (r:0 w:1)
	/// Proof: `Identity::IdentityTraitList` (`max_values`: None, `max_size`: Some(2088), added:
	/// 4563, mode: `MaxEncodedLen`)
	fn remove_long_identity_trait() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `3517`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(16_000_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
