//! Autogenerated weights for `pallet_certificate`
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
// --pallet=pallet_certificate
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

/// Weight functions needed for pallet_certificate.
pub trait WeightInfo {
	fn create_submission_entry() -> Weight;
	fn request_submission_assignment() -> Weight;
}

/// Weights for pallet_certificate using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Certificate::CertificateList` (r:1 w:1)
	/// Proof: `Certificate::CertificateList` (`max_values`: None, `max_size`: Some(97), added:
	/// 2572, mode: `MaxEncodedLen`)
	fn create_submission_entry() -> Weight {
		Weight::from_parts(13_000_000, 3562)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Certificate::CertificateList` (r:1 w:1)
	/// Proof: `Certificate::CertificateList` (`max_values`: None, `max_size`: Some(97), added:
	/// 2572, mode: `MaxEncodedLen`)
	fn request_submission_assignment() -> Weight {
		Weight::from_parts(15_000_000, 3562)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Certificate::CertificateList` (r:1 w:1)
	/// Proof: `Certificate::CertificateList` (`max_values`: None, `max_size`: Some(97), added:
	/// 2572, mode: `MaxEncodedLen`)
	fn create_submission_entry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3562`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3562)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Certificate::CertificateList` (r:1 w:1)
	/// Proof: `Certificate::CertificateList` (`max_values`: None, `max_size`: Some(97), added:
	/// 2572, mode: `MaxEncodedLen`)
	fn request_submission_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `203`
		//  Estimated: `3562`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(15_000_000, 3562)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
