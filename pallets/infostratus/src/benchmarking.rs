//! Benchmarking setup for pallet-infostratus
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_runtime::BoundedVec;
use frame_support::traits::Currency;
use crate::pallet::BalanceOf;
use sp_runtime::traits::Bounded;
use scale_info::prelude::format;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
    fn create_submission_entry() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let resource = BoundedVec::<u8, T::MaxSize>::try_from(b"BENCHMARK_RESOURCE".to_vec()).unwrap();
        // Ensure caller has enough balance
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
		#[extrinsic_call]
        create_submission_entry(RawOrigin::Signed(caller.clone()), resource.clone());
        // Assert storage
        assert!(SubmissionsList::<T>::contains_key(&caller, &resource));
        Ok(())
	}

    #[benchmark]
    fn create_submission_entry_heavy_storage() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let target_resource = BoundedVec::<u8, T::MaxSize>::try_from(b"FINAL_RESOURCE".to_vec()).unwrap();
        
        // Ensure caller has enough balance
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
        
        // Create 100,000 entries to test heavy storage conditions
        for i in 0..100_000 {
            let loop_resource = BoundedVec::<u8, T::MaxSize>::try_from(
                format!("RESOURCE_{}", i).as_bytes().to_vec()
            ).unwrap();
            
            Pallet::<T>::create_submission_entry(
                RawOrigin::Signed(caller.clone()).into(), 
                loop_resource
            )?;
        }
        
        #[extrinsic_call]
        create_submission_entry(RawOrigin::Signed(caller.clone()), target_resource.clone());
        
        // Assert storage for the final entry
        assert!(SubmissionsList::<T>::contains_key(&caller, &target_resource));
        
        Ok(())
    }

	#[benchmark]
    fn request_submission_assignment() -> Result<(), BenchmarkError> {
        let poster: T::AccountId = account("poster", 0, 0);
        let assignee: T::AccountId = whitelisted_caller();
        let resource = BoundedVec::<u8, T::MaxSize>::try_from(b"BENCHMARK_RESOURCE".to_vec()).unwrap();
        // Ensure both have enough balance
        T::Currency::make_free_balance_be(&poster, BalanceOf::<T>::max_value());
        T::Currency::make_free_balance_be(&assignee, BalanceOf::<T>::max_value());
        // Poster creates submission
        SubmissionsList::<T>::insert(&poster, &resource, false);
		#[extrinsic_call]
        request_submission_assignment(RawOrigin::Signed(assignee.clone()), poster.clone(), resource.clone());
        // Assert storage
        assert!(AssignmentsList::<T>::contains_key(&assignee, &resource));
        assert!(SubmissionsList::<T>::contains_key(&poster, &resource));
        Ok(())
	}

    #[benchmark]
    fn request_submission_assignment_heavy_storage() -> Result<(), BenchmarkError> {
        let poster: T::AccountId = account("poster", 0, 0);
        let assignee: T::AccountId = whitelisted_caller();
        let target_resource = BoundedVec::<u8, T::MaxSize>::try_from(b"FINAL_RESOURCE".to_vec()).unwrap();
        
        // Ensure both have enough balance
        T::Currency::make_free_balance_be(&poster, BalanceOf::<T>::max_value());
        T::Currency::make_free_balance_be(&assignee, BalanceOf::<T>::max_value());
        
        // Create many entries from the poster
        for i in 0..100_000 {
            let loop_resource = BoundedVec::<u8, T::MaxSize>::try_from(
                format!("RESOURCE_{}", i).as_bytes().to_vec()
            ).unwrap();
            
            Pallet::<T>::create_submission_entry(
                RawOrigin::Signed(poster.clone()).into(), 
                loop_resource.clone()
            )?;
            
            // Assign half of them to create heavy storage on the assignment side too
            if i % 2 == 0 {
                Pallet::<T>::request_submission_assignment(
                    RawOrigin::Signed(assignee.clone()).into(),
                    poster.clone(),
                    loop_resource
                )?;
            }
        }
        
        // Create the target submission
        Pallet::<T>::create_submission_entry(
            RawOrigin::Signed(poster.clone()).into(), 
            target_resource.clone()
        )?;
        
        #[extrinsic_call]
        request_submission_assignment(
            RawOrigin::Signed(assignee.clone()), 
            poster.clone(), 
            target_resource.clone()
        );
        
        // Assert storage for the target assignment
        assert!(AssignmentsList::<T>::contains_key(&assignee, &target_resource));
        assert!(SubmissionsList::<T>::contains_key(&poster, &target_resource));
        
        Ok(())
    }

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
