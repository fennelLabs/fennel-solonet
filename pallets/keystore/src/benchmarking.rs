#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_runtime::BoundedVec;
use scale_info::prelude::{format, vec};

#[benchmarks(
    where
        <T as frame_system::Config>::RuntimeEvent: From<Event<T>>,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
    fn announce_key() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(b"fingerprint".to_vec()).unwrap();
        let location = BoundedVec::<u8, T::MaxSize>::try_from(b"location".to_vec()).unwrap();
		#[extrinsic_call]
        announce_key(RawOrigin::Signed(caller.clone()), fingerprint.clone(), location.clone());
        // Storage and event check
        assert_eq!(IssuedKeys::<T>::get(&caller, &fingerprint), Some(location));
        frame_system::Pallet::<T>::assert_last_event(Event::KeyAnnounced { key: fingerprint, who: caller }.into());
        Ok(())
	}

    #[benchmark]
    fn announce_a_whole_lotta_keys() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(b"final_fingerprint".to_vec()).unwrap();
        let location = BoundedVec::<u8, T::MaxSize>::try_from(b"final_location".to_vec()).unwrap();
        
        // Create 100,000 keys in storage to test performance under heavy load
        for i in 0..100_000 {
            let loop_fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(
                format!("fingerprint{}", i).as_bytes().to_vec(),
            ).unwrap();
            let loop_location = BoundedVec::<u8, T::MaxSize>::try_from(
                format!("location{}", i).as_bytes().to_vec(),
            ).unwrap();
            
            Pallet::<T>::announce_key(
                RawOrigin::Signed(caller.clone()).into(), 
                loop_fingerprint, 
                loop_location
            )?;
        }
        
        #[extrinsic_call]
        announce_key(RawOrigin::Signed(caller.clone()), fingerprint.clone(), location.clone());
        
        // Verify storage and events
        assert_eq!(IssuedKeys::<T>::get(&caller, &fingerprint), Some(location));
        frame_system::Pallet::<T>::assert_last_event(Event::KeyAnnounced { key: fingerprint, who: caller }.into());
        
        Ok(())
    }

    #[benchmark]
    fn announce_key_with_long_vectors() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        // Create large 1000-byte vectors
        let fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(vec![0u8; 1000]).unwrap();
        let location = BoundedVec::<u8, T::MaxSize>::try_from(vec![1u8; 1000]).unwrap();
        
        #[extrinsic_call]
        announce_key(RawOrigin::Signed(caller.clone()), fingerprint.clone(), location.clone());
        
        // Verify storage and events
        assert_eq!(IssuedKeys::<T>::get(&caller, &fingerprint), Some(location));
        frame_system::Pallet::<T>::assert_last_event(Event::KeyAnnounced { key: fingerprint, who: caller }.into());
        
        Ok(())
    }

    #[benchmark]
    fn announce_a_bunch_of_long_keys() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        
        // Create 100,000 keys with long location values
        for i in 0..100_000 {
            let loop_fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(
                format!("fingerprint{}", i).as_bytes().to_vec(),
            ).unwrap();
            let loop_location = BoundedVec::<u8, T::MaxSize>::try_from(vec![i as u8 % 255; 1000]).unwrap();
            
            Pallet::<T>::announce_key(
                RawOrigin::Signed(caller.clone()).into(), 
                loop_fingerprint, 
                loop_location
            )?;
        }
        
        // Create the test key with large vectors
        let fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(vec![0u8; 1000]).unwrap();
        let location = BoundedVec::<u8, T::MaxSize>::try_from(vec![1u8; 1000]).unwrap();
        
        #[extrinsic_call]
        announce_key(RawOrigin::Signed(caller.clone()), fingerprint.clone(), location.clone());
        
        // Verify storage and events
        assert_eq!(IssuedKeys::<T>::get(&caller, &fingerprint), Some(location));
        frame_system::Pallet::<T>::assert_last_event(Event::KeyAnnounced { key: fingerprint, who: caller }.into());
        
        Ok(())
    }

	#[benchmark]
    fn revoke_key() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        
        let fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(b"fingerprint".to_vec()).unwrap();
        let location = BoundedVec::<u8, T::MaxSize>::try_from(b"location".to_vec()).unwrap();
        // Pre-insert key
        IssuedKeys::<T>::insert(&caller, &fingerprint, &location);
		#[extrinsic_call]
        revoke_key(RawOrigin::Signed(caller.clone()), fingerprint.clone());
        // Storage and event check
        assert_eq!(IssuedKeys::<T>::get(&caller, &fingerprint), None);
        frame_system::Pallet::<T>::assert_last_event(Event::KeyRevoked { key: fingerprint, who: caller }.into());
        Ok(())
	}

    #[benchmark]
    fn revoke_one_of_many_keys() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        
        // Add a special key to revoke
        let target_fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(b"target_key".to_vec()).unwrap();
        let target_location = BoundedVec::<u8, T::MaxSize>::try_from(b"target_location".to_vec()).unwrap();
        IssuedKeys::<T>::insert(&caller, &target_fingerprint, &target_location);
        
        // Create 100,000 additional keys
        for i in 0..100_000 {
            let loop_fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(
                format!("key{}", i).as_bytes().to_vec(),
            ).unwrap();
            let loop_location = BoundedVec::<u8, T::MaxSize>::try_from(vec![0; 32]).unwrap();
            
            IssuedKeys::<T>::insert(&caller, &loop_fingerprint, &loop_location);
        }
        
        #[extrinsic_call]
        revoke_key(RawOrigin::Signed(caller.clone()), target_fingerprint.clone());
        
        // Verify key was removed
        assert_eq!(IssuedKeys::<T>::get(&caller, &target_fingerprint), None);
        frame_system::Pallet::<T>::assert_last_event(Event::KeyRevoked { key: target_fingerprint, who: caller }.into());
        
        Ok(())
    }

	#[benchmark]
    fn issue_encryption_key() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let key = [0u8; 32];
		#[extrinsic_call]
        issue_encryption_key(RawOrigin::Signed(caller.clone()), key);
        // Storage and event check
        assert_eq!(IssuedEncryptionKeys::<T>::get(&caller), Some(key));
        frame_system::Pallet::<T>::assert_last_event(Event::EncryptionKeyIssued { who: caller }.into());
        Ok(())
	}

    #[benchmark]
    fn issue_a_ton_of_encryption_keys() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        
        // Override the encryption key 100,000 times
        // The storage will only keep the latest one, but this tests the performance
        // when there are many sequential writes
        for i in 0..100_000 {
            let temp_key = [i as u8 % 255; 32];
            Pallet::<T>::issue_encryption_key(
                RawOrigin::Signed(caller.clone()).into(),
                temp_key
            )?;
        }
        
        // Now issue the final key we'll measure
        let final_key = [42u8; 32];
        
        #[extrinsic_call]
        issue_encryption_key(RawOrigin::Signed(caller.clone()), final_key);
        
        // Verify the key was stored
        assert_eq!(IssuedEncryptionKeys::<T>::get(&caller), Some(final_key));
        frame_system::Pallet::<T>::assert_last_event(Event::EncryptionKeyIssued { who: caller }.into());
        
        Ok(())
    }

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
