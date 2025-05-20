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
        let name = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"TEST".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create parameter name"))?;
		let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), name.clone(), 0);

        assert!(SignalParameterList::<T>::contains_key(caller.clone(), name.clone()));
        assert_eq!(SignalParameterList::<T>::get(caller.clone(), name.clone()), 0);
        assert_last_event::<T>(Event::SignalParameterSet { who: caller }.into());
		Ok(())
	}

    // Added to match our comprehensive benchmarking approach
    #[benchmark]
    fn set_signal_parameter_large_input() -> Result<(), BenchmarkError> {
        // Create vector with 1000 bytes to match aggressive testing
        let name = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![0u8; 1000])
            .map_err(|_| BenchmarkError::Stop("Failed to create large parameter name - MaxSize might be too small"))?;
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
        let target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"TEST".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create target"))?;
		let caller: T::AccountId = whitelisted_caller();
		T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());

        // Create 100,000 signals to match original benchmarks
        for i in 0..100_000 {
            let loop_target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("TEST{}", i).as_bytes().to_vec()
            ).map_err(|_| BenchmarkError::Stop("Failed to create loop target"))?;
            
            Signal::<T>::send_rating_signal(
                RawOrigin::Signed(caller.clone()).into(),
                loop_target,
                3
            )?;
        }

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone(), 0);

		assert!(RatingSignalList::<T>::contains_key(caller.clone(), target.clone()));
        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 0);
        frame_system::Pallet::<T>::assert_has_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::SignalLock { account: caller.clone(), amount: T::LockPrice::get().into() }).into());
        frame_system::Pallet::<T>::assert_last_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::RatingSignalSent { who: caller.clone() }).into());
		Ok(())
	}

    // Added to match our comprehensive benchmarking approach
    #[benchmark]
    fn send_rating_signal_large_input() -> Result<(), BenchmarkError> {
        // Create 1000-byte target for aggressive testing
        let target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![1u8; 1000])
            .map_err(|_| BenchmarkError::Stop("Failed to create large target - MaxSize might be too small"))?;
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
        let target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"TEST".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create target"))?;
		let caller: T::AccountId = whitelisted_caller();
		T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
        
        // Generate 100,000 signals to match original benchmarks
        for i in 0..100_000 {
            let loop_target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("TEST{}", i).as_bytes().to_vec()
            ).map_err(|_| BenchmarkError::Stop("Failed to create loop target"))?;
            
            Signal::<T>::send_rating_signal(
                RawOrigin::Signed(caller.clone()).into(),
                loop_target,
                3
            )?;
        }
        
        // Create base rating signal
        Signal::<T>::send_rating_signal(
            RawOrigin::Signed(caller.clone()).into(), 
            target.clone(), 
            3
        )?;

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), target.clone(), 1);

        assert_eq!(RatingSignalList::<T>::get(caller.clone(), target.clone()), 1);
        frame_system::Pallet::<T>::assert_has_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::SignalLockExtended { account: caller.clone(), amount: T::LockPrice::get().into() }).into());
        frame_system::Pallet::<T>::assert_last_event(<T as pallet::Config>::RuntimeEvent::from(Event::<T>::RatingSignalUpdated { who: caller.clone() }).into());
		Ok(())
	}

	#[benchmark]
	fn revoke_rating_signal() -> Result<(), BenchmarkError> {
        let target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"TEST".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create target"))?;
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
        
        // Create 100,000 rating signals to match original benchmarks
        for i in 0..100_000 {
            let loop_target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("TEST{}", i).as_bytes().to_vec()
            ).map_err(|_| BenchmarkError::Stop("Failed to create loop target"))?;
            
            Signal::<T>::send_rating_signal(
                RawOrigin::Signed(caller.clone()).into(),
                loop_target,
                2
            )?;
        }
        
        // Create base rating signal
        Signal::<T>::send_rating_signal(
            RawOrigin::Signed(caller.clone()).into(), 
            target.clone(), 
            2
        )?;

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
        let signal = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"TEST".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create signal"))?;
		let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

        // Create 100,000 signals to match original benchmarks
        for _ in 0..100_000 {
            Signal::<T>::send_signal(
                RawOrigin::Signed(caller.clone()).into(), 
                signal.clone()
            )?;
        }

		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), signal.clone());

        assert_last_event::<T>(Event::SignalSent { signal, who: caller }.into());
		Ok(())
	}
	
    // Added to match our comprehensive benchmarking approach
    #[benchmark]
    fn send_signal_large_input() -> Result<(), BenchmarkError> {
        // Create 1000-byte signal for aggressive testing
        let signal = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![2u8; 1000])
            .map_err(|_| BenchmarkError::Stop("Failed to create large signal - MaxSize might be too small"))?;
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

        #[extrinsic_call]
        send_signal(RawOrigin::Signed(caller.clone()), signal.clone());

        assert_last_event::<T>(Event::SignalSent { signal, who: caller }.into());
        Ok(())
    }

	#[benchmark]
	fn send_service_signal() -> Result<(), BenchmarkError> {
        let service_identifier = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"TEST".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create service identifier"))?;
        let url = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(b"TEST".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create URL"))?;
		let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

        // Create 100,000 service signals to match original benchmarks
        for i in 0..100_000 {
            let loop_service = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("SERVICE{}", i).as_bytes().to_vec(),
            ).map_err(|_| BenchmarkError::Stop("Failed to create loop service identifier"))?;
            
            let loop_url = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("URL{}", i).as_bytes().to_vec(),
            ).map_err(|_| BenchmarkError::Stop("Failed to create loop URL"))?;
            
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

    // Added to match our comprehensive benchmarking approach
    #[benchmark]
    fn send_service_signal_large_input() -> Result<(), BenchmarkError> {
        // Create 500-byte vectors for each input (total 1000 bytes)
        let service_identifier = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![3u8; 500])
            .map_err(|_| BenchmarkError::Stop("Failed to create large service identifier - MaxSize might be too small"))?;
        let url = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![4u8; 500])
            .map_err(|_| BenchmarkError::Stop("Failed to create large URL - MaxSize might be too small"))?;
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::from(10_000u32));

        #[extrinsic_call]
        send_service_signal(RawOrigin::Signed(caller.clone()), service_identifier.clone(), url.clone());

        assert_last_event::<T>(Event::ServiceSignalSent { service_identifier, url, who: caller }.into());
        Ok(())
    }

	impl_benchmark_test_suite!(Signal, crate::mock::new_test_ext(), crate::mock::Test);
}
