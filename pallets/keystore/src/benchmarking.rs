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
        let fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(b"fingerprint".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create fingerprint"))?;
        let location = BoundedVec::<u8, T::MaxSize>::try_from(b"location".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create location"))?;
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
        let fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(b"fingerprint".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create fingerprint"))?;
        let location = BoundedVec::<u8, T::MaxSize>::try_from(b"location".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create location"))?;
        
        // Create 100,000 keys in storage to match original benchmarks
        // This is an aggressive test for heavy load conditions
        for i in 0..100_000 {
            let loop_fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(
                format!("fingerprint{}", i).as_bytes().to_vec(),
            ).map_err(|_| BenchmarkError::Stop("Failed to create loop fingerprint"))?;
            
            let loop_location = BoundedVec::<u8, T::MaxSize>::try_from(
                format!("location{}", i).as_bytes().to_vec(),
            ).map_err(|_| BenchmarkError::Stop("Failed to create loop location"))?;
            
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
        
        // Try with 1000-byte vectors as in the original benchmarks
        // Using proper error handling if the BoundedVec can't accommodate this size
        let fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(vec![0u8; 1000])
            .map_err(|_| BenchmarkError::Stop("Failed to create fingerprint - MaxSize might be too small"))?;
        let location = BoundedVec::<u8, T::MaxSize>::try_from(vec![1u8; 1000])
            .map_err(|_| BenchmarkError::Stop("Failed to create location - MaxSize might be too small"))?;
        
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
        
        // Create 100,000 keys with long location values as in the original benchmarks
        for i in 0..100_000 {
            let loop_fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(
                format!("fingerprint{}", i).as_bytes().to_vec(),
            ).map_err(|_| BenchmarkError::Stop("Failed to create loop fingerprint"))?;
            
            // Use 1000-byte vectors for long values
            let loop_location = BoundedVec::<u8, T::MaxSize>::try_from(vec![i as u8 % 255; 1000])
                .map_err(|_| BenchmarkError::Stop("Failed to create loop location - MaxSize might be too small"))?;
            
            Pallet::<T>::announce_key(
                RawOrigin::Signed(caller.clone()).into(), 
                loop_fingerprint, 
                loop_location
            )?;
        }
        
        // Create the test key with large vectors (1000 bytes)
        let fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(vec![0u8; 1000])
            .map_err(|_| BenchmarkError::Stop("Failed to create fingerprint - MaxSize might be too small"))?;
        let location = BoundedVec::<u8, T::MaxSize>::try_from(vec![1u8; 1000])
            .map_err(|_| BenchmarkError::Stop("Failed to create location - MaxSize might be too small"))?;
        
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
        
        let fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(b"somekey".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create fingerprint"))?;
        
        let location = BoundedVec::<u8, T::MaxSize>::try_from(vec![0u8; 32])
            .map_err(|_| BenchmarkError::Stop("Failed to create location"))?;
        
        // Pre-insert key
        Pallet::<T>::announce_key(
            RawOrigin::Signed(caller.clone()).into(),
            fingerprint.clone(),
            location
        )?;
        
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
        
        // Create 100,000 additional keys as in the original benchmark
        for i in 0..100_000 {
            let loop_fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(
                format!("key{}", i).as_bytes().to_vec(),
            ).map_err(|_| BenchmarkError::Stop("Failed to create loop fingerprint"))?;
            
            let loop_location = BoundedVec::<u8, T::MaxSize>::try_from(vec![0; 32])
                .map_err(|_| BenchmarkError::Stop("Failed to create loop location"))?;
            
            Pallet::<T>::announce_key(
                RawOrigin::Signed(caller.clone()).into(),
                loop_fingerprint,
                loop_location
            )?;
        }
        
        // Add a special key to revoke
        let target_fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(b"somekey".to_vec())
            .map_err(|_| BenchmarkError::Stop("Failed to create target fingerprint"))?;
        
        let target_location = BoundedVec::<u8, T::MaxSize>::try_from(vec![0; 32])
            .map_err(|_| BenchmarkError::Stop("Failed to create target location"))?;
        
        Pallet::<T>::announce_key(
            RawOrigin::Signed(caller.clone()).into(),
            target_fingerprint.clone(),
            target_location
        )?;
        
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
        
        // Override the encryption key 100,000 times as in the original benchmark
        for i in 0..100_000 {
            let temp_key = [i as u8 % 255; 32];
            Pallet::<T>::issue_encryption_key(
                RawOrigin::Signed(caller.clone()).into(),
                temp_key
            )?;
        }
        
        // Now issue the final key we'll measure
        let final_key = [0u8; 32];
        
        #[extrinsic_call]
        issue_encryption_key(RawOrigin::Signed(caller.clone()), final_key);
        
        // Verify the key was stored
        assert_eq!(IssuedEncryptionKeys::<T>::get(&caller), Some(final_key));
        frame_system::Pallet::<T>::assert_last_event(Event::EncryptionKeyIssued { who: caller }.into());
        
        Ok(())
    }

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
