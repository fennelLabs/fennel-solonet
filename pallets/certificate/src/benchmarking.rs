//! Benchmarking setup for pallet-certificate
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Certificate;
use frame_benchmarking::v2::*;
use frame_support::{traits::Currency, sp_runtime::traits::Bounded};
use frame_system::RawOrigin;

pub fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks]
mod benchmarks {
	use super::*;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[benchmark]
	fn send_certificate() -> Result<(), BenchmarkError> {
		let recipient: T::AccountId = account("James", 0, 0);
		let sender: T::AccountId = whitelisted_caller();

		T::Currency::make_free_balance_be(&sender, BalanceOf::<T>::max_value());

		#[extrinsic_call]
		send_certificate(RawOrigin::Signed(sender.clone()), recipient.clone());

		assert!(CertificateList::<T>::contains_key(sender.clone(), recipient.clone()));
		assert_last_event::<T>(Event::CertificateSent { sender, recipient }.into());

		Ok(())
	}

	#[benchmark]
	fn revoke_certificate() -> Result<(), BenchmarkError> {
		let recipient: T::AccountId = account("Montgomery", 0, 0);
		let sender: T::AccountId = whitelisted_caller();

		T::Currency::make_free_balance_be(&sender, BalanceOf::<T>::max_value());
		// Use direct call for setup
		Certificate::<T>::send_certificate(RawOrigin::Signed(sender.clone()).into(), recipient.clone())?;

		#[extrinsic_call]
		revoke_certificate(RawOrigin::Signed(sender.clone()), recipient.clone());

		assert!(CertificateList::<T>::contains_key(sender.clone(), recipient.clone()));
		assert!(!CertificateList::<T>::get(sender.clone(), recipient.clone()));
		assert_last_event::<T>(Event::CertificateRevoked { sender, recipient }.into());
		Ok(())
	}

	#[benchmark]
	fn send_certificate_heavy_storage() -> Result<(), BenchmarkError> {
		let sender: T::AccountId = whitelisted_caller();

		T::Currency::make_free_balance_be(&sender, BalanceOf::<T>::max_value());

		// Generate a larger set of certificates.
		for i in 0..10000 {
			let target: T::AccountId = account("target", i, 0);
			Certificate::<T>::send_certificate(RawOrigin::Signed(sender.clone()).into(), target.clone())?;
		}

		let recipient: T::AccountId = account("Montgomery", 0, 0);
		
		#[extrinsic_call]
		send_certificate(RawOrigin::Signed(sender.clone()), recipient.clone());

		assert!(CertificateList::<T>::contains_key(sender.clone(), recipient.clone()));
		assert_last_event::<T>(Event::CertificateSent { sender, recipient }.into());
		
		Ok(())
	}

	#[benchmark]
	fn revoke_certificate_heavy_storage() -> Result<(), BenchmarkError> {
		let sender: T::AccountId = whitelisted_caller();

		T::Currency::make_free_balance_be(&sender, BalanceOf::<T>::max_value());

		// Generate a larger set of certificates.
		for i in 0..10000 {
			let target: T::AccountId = account("target", i, 0);
			Certificate::<T>::send_certificate(RawOrigin::Signed(sender.clone()).into(), target.clone())?;
		}

		let recipient: T::AccountId = account("Montgomery", 0, 0);
		Certificate::<T>::send_certificate(RawOrigin::Signed(sender.clone()).into(), recipient.clone())?;
		
		#[extrinsic_call]
		revoke_certificate(RawOrigin::Signed(sender.clone()), recipient.clone());

		assert!(CertificateList::<T>::contains_key(sender.clone(), recipient.clone()));
		assert!(!CertificateList::<T>::get(sender.clone(), recipient.clone()));
		assert_last_event::<T>(Event::CertificateRevoked { sender, recipient }.into());
		
		Ok(())
	}

	impl_benchmark_test_suite!(Certificate, crate::mock::new_test_ext(), crate::mock::Test);
}
