//! Benchmarking setup for pallet-signal
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Signal;

use frame_benchmarking::{v2::*};
use frame_support::{traits::Currency, BoundedVec};
use frame_support::traits::Get;
use frame_system::RawOrigin;
use frame_support::sp_runtime::traits::Bounded;
use scale_info::prelude::{format, vec};

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
		let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

        // Pre-populate with 100,000 entries to test performance at scale
        for i in 0..100_000 {
            let loop_name = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("PARAM{}", i).as_bytes().to_vec()
            ).unwrap();
            SignalParameterList::<T>::insert(&caller, &loop_name, i as u8 % 255);
        }

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), name.clone(), 42);

        assert!(SignalParameterList::<T>::contains_key(caller.clone(), name.clone()));
        assert_eq!(SignalParameterList::<T>::get(caller.clone(), name.clone()), 42);
        assert_last_event::<T>(Event::SignalParameterSet { who: caller }.into());
		Ok(())
	}

    #[benchmark]
    fn set_signal_parameter_large_input() -> Result<(), BenchmarkError> {
        // Create large 1000-byte name
        let name = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![0u8; 1000]).unwrap();
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

        #[extrinsic_call]
        set_signal_parameter(RawOrigin::Signed(caller.clone()), name.clone(), 42);

        assert!(SignalParameterList::<T>::contains_key(caller.clone(), name.clone()));
        assert_eq!(SignalParameterList::<T>::get(caller.clone(), name.clone()), 42);
        assert_last_event::<T>(Event::SignalParameterSet { who: caller }.into());
        Ok(())
    }

	#[benchmark]
	fn send_rating_signal() -> Result<(), BenchmarkError> {
        let target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"TARGET".to_vec()).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());

        // Pre-populate with 100,000 entries to test performance at scale
        for i in 0..100_000 {
            let loop_target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("TARGET{}", i).as_bytes().to_vec()
            ).unwrap();
            // We can't actually create 100k rating signals as they'd lock currency
            // but we can still load storage with other types of signals
            SignalParameterList::<T>::insert(&caller, &loop_target, i as u8 % 255);
        }

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone(), 5);

		assert!(RatingSignalList::<T>::contains_key(caller.clone(), target.clone()));
        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 5);
        frame_system::Pallet::<T>::assert_has_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::SignalLock { account: caller.clone(), amount: T::LockPrice::get().into() }).into());
        frame_system::Pallet::<T>::assert_last_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::RatingSignalSent { who: caller.clone() }).into());
		Ok(())
	}

    #[benchmark]
    fn send_rating_signal_large_input() -> Result<(), BenchmarkError> {
        // Create large 1000-byte target
        let target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![1u8; 1000]).unwrap();
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());

        #[extrinsic_call]
        send_rating_signal(RawOrigin::Signed(caller.clone()), target.clone(), 5);

        assert!(RatingSignalList::<T>::contains_key(caller.clone(), target.clone()));
        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 5);
        frame_system::Pallet::<T>::assert_has_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::SignalLock { account: caller.clone(), amount: T::LockPrice::get().into() }).into());
        frame_system::Pallet::<T>::assert_last_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::RatingSignalSent { who: caller.clone() }).into());
        Ok(())
    }

	#[benchmark]
	fn update_rating_signal() -> Result<(), BenchmarkError> {
        let target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"TARGET".to_vec()).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
        
        // Create base rating signal
        Signal::<T>::send_rating_signal(RawOrigin::Signed(caller.clone()).into(), target.clone(), 5)?;
        
        // Add 100,000 entries to test performance with heavy storage
        for i in 0..100_000 {
            let loop_target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("TARGET{}", i).as_bytes().to_vec()
            ).unwrap();
            // We can't create 100k actual rating signals due to currency locks
            // Using SignalParameterList to populate storage instead
            SignalParameterList::<T>::insert(&caller, &loop_target, i as u8 % 255);
        }
        
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());

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
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
        
        // Create base rating signal
        Signal::<T>::send_rating_signal(RawOrigin::Signed(caller.clone()).into(), target.clone(), 5)?;
        
        // Add 100,000 entries to test performance with heavy storage
        for i in 0..100_000 {
            let loop_target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("TARGET{}", i).as_bytes().to_vec()
            ).unwrap();
            // We can't create 100k actual rating signals due to currency locks
            // Using SignalParameterList to populate storage instead
            SignalParameterList::<T>::insert(&caller, &loop_target, i as u8 % 255);
        }
        
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone());

        // Check for full removal, not just zeroing
        assert!(!RatingSignalList::<T>::contains_key(caller.clone(), target.clone()));
        frame_system::Pallet::<T>::assert_has_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::SignalUnlock { account: caller.clone() }).into());
        frame_system::Pallet::<T>::assert_last_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::RatingSignalRevoked { who: caller.clone() }).into());
		Ok(())
	}

	#[benchmark]
	fn send_signal() -> Result<(), BenchmarkError> {
        let signal = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"SIGNAL".to_vec()).unwrap();
		let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

        // Pre-populate with 100,000 entries to test performance at scale
        for i in 0..100_000 {
            let loop_signal = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("SIGNAL{}", i).as_bytes().to_vec()
            ).unwrap();
            Signal::<T>::send_signal(RawOrigin::Signed(caller.clone()).into(), loop_signal)?;
        }

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), signal.clone());

        assert_last_event::<T>(Event::SignalSent { signal, who: caller }.into());
		Ok(())
	}

    #[benchmark]
    fn send_signal_large_input() -> Result<(), BenchmarkError> {
        // Create large 1000-byte signal
        let signal = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![2u8; 1000]).unwrap();
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

        #[extrinsic_call]
        send_signal(RawOrigin::Signed(caller.clone()), signal.clone());

        assert_last_event::<T>(Event::SignalSent { signal, who: caller }.into());
        Ok(())
    }

	#[benchmark]
	fn send_service_signal() -> Result<(), BenchmarkError> {
        let service_identifier = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"SERVICE".to_vec()).unwrap();
        let url = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"URL".to_vec()).unwrap();
		let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

        // Pre-populate with 100,000 entries to test performance at scale
        for i in 0..100_000 {
            let loop_service = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("SERVICE{}", i).as_bytes().to_vec()
            ).unwrap();
            let loop_url = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("URL{}", i).as_bytes().to_vec()
            ).unwrap();
            Signal::<T>::send_service_signal(
                RawOrigin::Signed(caller.clone()).into(), 
                loop_service, 
                loop_url
            )?;
        }

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), service_identifier.clone(), url.clone());

        assert_last_event::<T>(Event::ServiceSignalSent { service_identifier, url, who: caller }.into());
		Ok(())
	}

    #[benchmark]
    fn send_service_signal_large_input() -> Result<(), BenchmarkError> {
        // Create large 1000-byte inputs
        let service_identifier = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![3u8; 1000]).unwrap();
        let url = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![4u8; 1000]).unwrap();
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

        #[extrinsic_call]
        send_service_signal(RawOrigin::Signed(caller.clone()), service_identifier.clone(), url.clone());

        assert_last_event::<T>(Event::ServiceSignalSent { service_identifier, url, who: caller }.into());
        Ok(())
    }

	impl_benchmark_test_suite!(Signal, crate::mock::new_test_ext(), crate::mock::Test);
}
