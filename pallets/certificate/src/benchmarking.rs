//! Benchmarking setup for pallet-certificate
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Certificate;
use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::{traits::Currency, sp_runtime::traits::Bounded};
use frame_system::RawOrigin;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	benchmark_account(name, 0, 0)
}

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
	RawOrigin::Signed(get_account::<T>(name))
}

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
		let target = get_account::<T>("James");
		let caller = get_origin::<T>("Spock");
		let caller_account: T::AccountId = get_account::<T>("Spock");

		T::Currency::make_free_balance_be(&caller_account, BalanceOf::<T>::max_value());

		#[extrinsic_call]
		_(caller, target);

		let caller_account_id: T::AccountId = get_account::<T>("Spock");
		let target_account_id: T::AccountId = get_account::<T>("James");
		assert!(CertificateList::<T>::contains_key(caller_account_id.clone(), target_account_id.clone()));
		assert_last_event::<T>(Event::CertificateSent { sender: caller_account_id, recipient: target_account_id }.into());

		Ok(())
	}

	#[benchmark]
	fn revoke_certificate() -> Result<(), BenchmarkError> {
		let target = get_account::<T>("Montgomery");
		let caller = get_origin::<T>("Spock");
		let caller_account: T::AccountId = get_account::<T>("Spock");

		T::Currency::make_free_balance_be(&caller_account, BalanceOf::<T>::max_value());
		// Use direct call for setup, not the macro shorthand
		Certificate::<T>::send_certificate(caller.clone().into(), target.clone())?;

		#[extrinsic_call]
		_(caller, target.clone());

		let caller_account_id: T::AccountId = get_account::<T>("Spock");
		let target_account_id: T::AccountId = get_account::<T>("Montgomery");
		assert!(CertificateList::<T>::contains_key(caller_account_id.clone(), target_account_id.clone()));
		assert!(!CertificateList::<T>::get(caller_account_id.clone(), target_account_id.clone()));
		assert_last_event::<T>(Event::CertificateRevoked { sender: caller_account_id, recipient: target_account_id }.into());
		Ok(())
	}

	impl_benchmark_test_suite!(Certificate, crate::mock::new_test_ext(), crate::mock::Test);
}
