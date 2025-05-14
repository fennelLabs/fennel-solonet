#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_runtime::BoundedVec;

#[benchmarks(
    where
        <T as frame_system::Config>::RuntimeEvent: From<Event<T>>,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
    fn announce_key() {
        let caller: T::AccountId = whitelisted_caller();
        let fingerprint = BoundedVec::<u8, T::MaxSize>::try_from(b"fingerprint".to_vec()).unwrap();
        let location = BoundedVec::<u8, T::MaxSize>::try_from(b"location".to_vec()).unwrap();
		#[extrinsic_call]
        announce_key(RawOrigin::Signed(caller.clone()), fingerprint.clone(), location.clone());
        // Storage and event check
        assert_eq!(IssuedKeys::<T>::get(&caller, &fingerprint), Some(location));
        frame_system::Pallet::<T>::assert_last_event(Event::KeyAnnounced { key: fingerprint, who: caller }.into());
	}

	#[benchmark]
    fn revoke_key() {
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
	}

	#[benchmark]
    fn issue_encryption_key() {
        let caller: T::AccountId = whitelisted_caller();
        let key = [0u8; 32];
		#[extrinsic_call]
        issue_encryption_key(RawOrigin::Signed(caller.clone()), key);
        // Storage and event check
        assert_eq!(IssuedEncryptionKeys::<T>::get(&caller), Some(key));
        frame_system::Pallet::<T>::assert_last_event(Event::EncryptionKeyIssued { who: caller }.into());
	}

	impl_benchmark_test_suite!(Keystore, crate::mock::new_test_ext(), crate::mock::Test);
}
