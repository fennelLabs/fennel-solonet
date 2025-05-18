//! FRAME benchmarking for pallet-identity.

#![cfg(feature = "runtime-benchmarks")]

use crate::{Config};
use frame_benchmarking::v2::*;
use scale_info::prelude::vec;

#[benchmarks(
    where
        T: Config<MaxSize = frame_support::traits::ConstU32<1024>>,
)]
mod benchmarks {
	use super::*;
    use crate::{Call, Pallet};
    use frame_system::RawOrigin;
    use frame_support::BoundedVec;
    use frame_benchmarking::{BenchmarkError, whitelisted_caller};
    use scale_info::prelude::format;

	#[benchmark]
	fn create_identity() -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
        Pallet::<T>::create_identity(RawOrigin::Signed(who.clone()));
        let last = Pallet::<T>::identity_number().saturating_sub(1);
        assert_eq!(Pallet::<T>::identity_list(last), Some(who));
		Ok(())
	}

	#[benchmark]
	fn revoke_identity() -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        Pallet::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        let last = Pallet::<T>::identity_number().saturating_sub(1);
		#[extrinsic_call]
        Pallet::<T>::revoke_identity(RawOrigin::Signed(who.clone()), last);
        // Check that the identity no longer exists in the IdentityList.
        assert!(Pallet::<T>::identity_list(last).is_none());
		Ok(())
	}

	#[benchmark]
    fn add_or_update_identity_trait(l: Linear<1, 1024>) -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        Pallet::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        let last = Pallet::<T>::identity_number().saturating_sub(1);
        let key: BoundedVec<u8, T::MaxSize> = vec![0u8; l as usize].try_into().unwrap();
        let val: BoundedVec<u8, T::MaxSize> = vec![1u8; l as usize].try_into().unwrap();
		#[extrinsic_call]
        Pallet::<T>::add_or_update_identity_trait(RawOrigin::Signed(who.clone()), last, key.clone(), val.clone());
        // Check that the identity no longer exists in the IdentityList.
        assert_eq!(Pallet::<T>::identity_trait_list(last, key), val);
		Ok(())
	}

	#[benchmark]
    fn remove_identity_trait(l: Linear<1, 1024>) -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        Pallet::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        let last = Pallet::<T>::identity_number().saturating_sub(1);
        let key: BoundedVec<u8, T::MaxSize> = vec![0u8; l as usize].try_into().unwrap();
        let val: BoundedVec<u8, T::MaxSize> = vec![1u8; l as usize].try_into().unwrap();
        Pallet::<T>::add_or_update_identity_trait(
            RawOrigin::Signed(who.clone()).into(),
            last,
            key.clone(),
            val,
		)?;

		#[extrinsic_call]
        Pallet::<T>::remove_identity_trait(RawOrigin::Signed(who.clone()), last, key.clone());
        assert_eq!(Pallet::<T>::identity_trait_list(last, key), BoundedVec::<u8, T::MaxSize>::default());
		Ok(())
	}

    #[benchmark]
    fn revoke_identity_heavy_storage() -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        
        // Create 1000 identities for heavy storage testing
        for _ in 0..1000 {
            Pallet::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        }
        
        // Get an identity ID to revoke - choose an index in the middle
        let identity_to_revoke = 500;
        
        #[extrinsic_call]
        Pallet::<T>::revoke_identity(RawOrigin::Signed(who.clone()), identity_to_revoke);
        
        // Verify that the identity has been revoked
        assert!(Pallet::<T>::identity_list(identity_to_revoke).is_none());
        
        Ok(())
    }

    #[benchmark]
    fn add_or_update_long_identity_trait() -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        Pallet::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        let identity_id = Pallet::<T>::identity_number().saturating_sub(1);
        
        // Create large 1000-byte key/value pairs
        let key: BoundedVec<u8, T::MaxSize> = vec![0u8; 1000].try_into().unwrap();
        let val: BoundedVec<u8, T::MaxSize> = vec![1u8; 1000].try_into().unwrap();
        
        #[extrinsic_call]
        Pallet::<T>::add_or_update_identity_trait(
            RawOrigin::Signed(who.clone()), 
            identity_id, 
            key.clone(), 
            val.clone()
        );
        
        // Verify the trait was set correctly
        assert_eq!(Pallet::<T>::identity_trait_list(identity_id, key.clone()), val);
        
        Ok(())
    }

    #[benchmark]
    fn add_or_update_many_identity_traits() -> Result<(), BenchmarkError> {
        // Create first identity with many traits
        let first_user: T::AccountId = whitelisted_caller();
        Pallet::<T>::create_identity(RawOrigin::Signed(first_user.clone()).into())?;
        let first_identity_id = Pallet::<T>::identity_number().saturating_sub(1);
        
        // Add 100,000 traits to create heavy storage condition
        for i in 0..100_000 {
            let key: BoundedVec<u8, T::MaxSize> = 
                format!("name{}", i).as_bytes().to_vec().try_into().unwrap();
            let val: BoundedVec<u8, T::MaxSize> = 
                format!("value{}", i).as_bytes().to_vec().try_into().unwrap();
                
            Pallet::<T>::add_or_update_identity_trait(
                RawOrigin::Signed(first_user.clone()).into(), 
                first_identity_id, 
                key, 
                val
            )?;
        }
        
        // Create a second identity and test adding a trait with heavy storage
        let second_user: T::AccountId = whitelisted_caller();
        Pallet::<T>::create_identity(RawOrigin::Signed(second_user.clone()).into())?;
        let second_identity_id = Pallet::<T>::identity_number().saturating_sub(1);
        
        let key: BoundedVec<u8, T::MaxSize> = vec![0u8; 1000].try_into().unwrap();
        let val: BoundedVec<u8, T::MaxSize> = vec![1u8; 1000].try_into().unwrap();
        
        #[extrinsic_call]
        Pallet::<T>::add_or_update_identity_trait(
            RawOrigin::Signed(second_user.clone()), 
            second_identity_id, 
            key.clone(), 
            val.clone()
        );
        
        // Verify the trait was set correctly
        assert_eq!(Pallet::<T>::identity_trait_list(second_identity_id, key), val);
        
        Ok(())
    }

    #[benchmark]
    fn remove_identity_trait_heavy_storage() -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        Pallet::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        let identity_id = Pallet::<T>::identity_number().saturating_sub(1);
        
        // Create 100,000 traits to test heavy storage
        for i in 0..100_000 {
            let key: BoundedVec<u8, T::MaxSize> = 
                format!("name{}", i).as_bytes().to_vec().try_into().unwrap();
            let val: BoundedVec<u8, T::MaxSize> = 
                format!("value{}", i).as_bytes().to_vec().try_into().unwrap();
                
            Pallet::<T>::add_or_update_identity_trait(
                RawOrigin::Signed(who.clone()).into(), 
                identity_id, 
                key, 
                val
            )?;
        }
        
        // Create a special key to remove
        let target_key: BoundedVec<u8, T::MaxSize> = vec![9u8; 1000].try_into().unwrap();
        let target_val: BoundedVec<u8, T::MaxSize> = vec![9u8; 1000].try_into().unwrap();
        
        // Add the special key
        Pallet::<T>::add_or_update_identity_trait(
            RawOrigin::Signed(who.clone()).into(), 
            identity_id, 
            target_key.clone(), 
            target_val
        )?;
        
        #[extrinsic_call]
        Pallet::<T>::remove_identity_trait(
            RawOrigin::Signed(who.clone()), 
            identity_id, 
            target_key.clone()
        );
        
        // Verify the trait was removed
        assert_eq!(
            Pallet::<T>::identity_trait_list(identity_id, target_key), 
            BoundedVec::<u8, T::MaxSize>::default()
        );
        
        Ok(())
    }

    #[benchmark]
    fn remove_long_identity_trait() -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        Pallet::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        let identity_id = Pallet::<T>::identity_number().saturating_sub(1);
        
        // Create a large 1000-byte key/value pair
        let key: BoundedVec<u8, T::MaxSize> = vec![0u8; 1000].try_into().unwrap();
        let val: BoundedVec<u8, T::MaxSize> = vec![1u8; 1000].try_into().unwrap();
        
        // Add the trait
        Pallet::<T>::add_or_update_identity_trait(
            RawOrigin::Signed(who.clone()).into(), 
            identity_id, 
            key.clone(), 
            val
        )?;
        
        #[extrinsic_call]
        Pallet::<T>::remove_identity_trait(
            RawOrigin::Signed(who.clone()), 
            identity_id, 
            key.clone()
        );
        
        // Verify the trait was removed
        assert_eq!(
            Pallet::<T>::identity_trait_list(identity_id, key), 
            BoundedVec::<u8, T::MaxSize>::default()
        );
        
        Ok(())
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
