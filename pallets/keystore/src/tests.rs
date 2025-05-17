use crate::{mock::*, Error, Event, IssuedKeys, IssuedEncryptionKeys};
use frame_support::{assert_ok, assert_noop};
use sp_core::ConstU32;
use frame_support::BoundedVec;

#[test]
fn announce_key_works_and_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        let fingerprint = BoundedVec::<u8, ConstU32<1024>>::try_from(b"Luke".to_vec()).unwrap();
        let location = BoundedVec::<u8, ConstU32<1024>>::try_from(b"Skywalker".to_vec()).unwrap();
        assert_ok!(Keystore::announce_key(RuntimeOrigin::signed(1), fingerprint.clone(), location.clone()));
        // Storage check
        assert_eq!(IssuedKeys::<Test>::get(1, &fingerprint), Some(location));
        // Event check
        System::assert_last_event(Event::KeyAnnounced { key: fingerprint, who: 1 }.into());
	});
}

#[test]
fn cannot_announce_duplicate_key() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        let fingerprint = BoundedVec::<u8, ConstU32<1024>>::try_from(b"Luke".to_vec()).unwrap();
        let location = BoundedVec::<u8, ConstU32<1024>>::try_from(b"Skywalker".to_vec()).unwrap();
        assert_ok!(Keystore::announce_key(RuntimeOrigin::signed(1), fingerprint.clone(), location.clone()));
        assert_noop!(Keystore::announce_key(RuntimeOrigin::signed(1), fingerprint.clone(), location), Error::<Test>::KeyExists);
	});
}

#[test]
fn revoke_key_works_and_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        let fingerprint = BoundedVec::<u8, ConstU32<1024>>::try_from(b"Luke".to_vec()).unwrap();
        let location = BoundedVec::<u8, ConstU32<1024>>::try_from(b"Skywalker".to_vec()).unwrap();
        assert_ok!(Keystore::announce_key(RuntimeOrigin::signed(1), fingerprint.clone(), location));
        assert_ok!(Keystore::revoke_key(RuntimeOrigin::signed(1), fingerprint.clone()));
        // Storage check
        assert_eq!(IssuedKeys::<Test>::get(1, &fingerprint), None);
        // Event check
        System::assert_last_event(Event::KeyRevoked { key: fingerprint, who: 1 }.into());
    });
}

#[test]
fn cannot_revoke_nonexistent_key() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let fingerprint = BoundedVec::<u8, ConstU32<1024>>::try_from(b"Luke".to_vec()).unwrap();
        assert_noop!(Keystore::revoke_key(RuntimeOrigin::signed(1), fingerprint), Error::<Test>::KeyDoesNotExist);
    });
}

#[test]
fn issue_encryption_key_works_and_emits_event() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let key = [0u8; 32];
        assert_ok!(Keystore::issue_encryption_key(RuntimeOrigin::signed(1), key));
        // Storage check
        assert_eq!(IssuedEncryptionKeys::<Test>::get(1), Some(key));
        // Event check
        System::assert_last_event(Event::EncryptionKeyIssued { who: 1 }.into());
	});
}
