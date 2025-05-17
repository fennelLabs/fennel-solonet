//! Benchmarking setup for pallet-signal
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Signal;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::{traits::Currency, BoundedVec};
use frame_support::traits::Get;
use frame_system::RawOrigin;
use frame_support::sp_runtime::traits::Bounded;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    benchmark_account(name, 0, 0)
}

// pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
//     RawOrigin::Signed(get_account::<T>(name))
// }

pub fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks]
mod benchmarks {
	use super::*;
	type DepositBalanceOf<T> = <<T as pallet::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	#[benchmark]
	fn set_signal_parameter() -> Result<(), BenchmarkError> {
        let name = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"PARAM".to_vec()).unwrap();
		let caller: T::AccountId = get_account::<T>("//Alice");
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), name.clone(), 42);

        assert!(SignalParameterList::<T>::contains_key(caller.clone(), name.clone()));
        assert_eq!(SignalParameterList::<T>::get(caller.clone(), name.clone()), 42);
        assert_last_event::<T>(Event::SignalParameterSet { who: caller }.into());
		Ok(())
	}

	#[benchmark]
	fn send_rating_signal() -> Result<(), BenchmarkError> {
        let target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"TARGET".to_vec()).unwrap();
		let caller: T::AccountId = get_account::<T>("//Alice");

		T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
        let balance = T::Currency::free_balance(&caller);
        frame_support::runtime_print!("send_rating_signal: balance before call: {:?}", balance);

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone(), 5);

		assert!(RatingSignalList::<T>::contains_key(caller.clone(), target.clone()));
        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 5);
        frame_system::Pallet::<T>::assert_has_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::SignalLock { account: caller.clone(), amount: T::LockPrice::get().into() }).into());
        frame_system::Pallet::<T>::assert_last_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::RatingSignalSent { who: caller.clone() }).into());
		Ok(())
	}

	#[benchmark]
	fn update_rating_signal() -> Result<(), BenchmarkError> {
        let target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"TARGET".to_vec()).unwrap();
		let caller: T::AccountId = get_account::<T>("//Alice");
		T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
        let balance = T::Currency::free_balance(&caller);
        frame_support::runtime_print!("update_rating_signal: balance before setup call: {:?}", balance);
        Signal::<T>::send_rating_signal(RawOrigin::Signed(caller.clone()).into(), target.clone(), 5)?;
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
        let balance = T::Currency::free_balance(&caller);
        frame_support::runtime_print!("update_rating_signal: balance before main call: {:?}", balance);

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone(), 9);

        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 9);
        frame_system::Pallet::<T>::assert_has_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::SignalLockExtended { account: caller.clone(), amount: T::LockPrice::get().into() }).into());
        frame_system::Pallet::<T>::assert_last_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::RatingSignalUpdated { who: caller.clone() }).into());
		Ok(())
	}

	#[benchmark]
	fn revoke_rating_signal() -> Result<(), BenchmarkError> {
        let target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"TARGET".to_vec()).unwrap();
        let caller: T::AccountId = get_account::<T>("//Alice");
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
        let balance = T::Currency::free_balance(&caller);
        frame_support::runtime_print!("revoke_rating_signal: balance before setup call: {:?}", balance);
        Signal::<T>::send_rating_signal(RawOrigin::Signed(caller.clone()).into(), target.clone(), 5)?;
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
        let balance = T::Currency::free_balance(&caller);
        frame_support::runtime_print!("revoke_rating_signal: balance before main call: {:?}", balance);

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone());

        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 0);
        frame_system::Pallet::<T>::assert_has_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::SignalUnlock { account: caller.clone() }).into());
        frame_system::Pallet::<T>::assert_last_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::RatingSignalRevoked { who: caller.clone() }).into());
		Ok(())
	}

	#[benchmark]
	fn send_signal() -> Result<(), BenchmarkError> {
        let signal = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"SIGNAL".to_vec()).unwrap();
		let caller: T::AccountId = get_account::<T>("//Alice");
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), signal.clone());

        assert_last_event::<T>(Event::SignalSent { signal, who: caller }.into());
		Ok(())
	}

	#[benchmark]
	fn send_service_signal() -> Result<(), BenchmarkError> {
        let service_identifier = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"SERVICE".to_vec()).unwrap();
        let url = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"URL".to_vec()).unwrap();
		let caller: T::AccountId = get_account::<T>("//Alice");
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), service_identifier.clone(), url.clone());

        assert_last_event::<T>(Event::ServiceSignalSent { service_identifier, url, who: caller }.into());
		Ok(())
	}

	impl_benchmark_test_suite!(Signal, crate::mock::new_test_ext(), crate::mock::Test);
}
