use crate::mock::{new_test_ext, Test, System, RuntimeOrigin};
use crate::{Error, Event, Pallet, IdentityNumber};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use crate as pallet_identity;

#[test]
fn create_identity_works_and_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_ok!(Pallet::<Test>::create_identity(RuntimeOrigin::signed(1)));
        System::assert_last_event(Event::IdentityCreated { identity_id: 0, owner: 1 }.into());
        assert_eq!(Pallet::<Test>::identity_list(0).unwrap(), 1);
	});
}

#[test]
fn create_identity_increments_identity_number() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_ok!(Pallet::<Test>::create_identity(RuntimeOrigin::signed(1)));
        assert_ok!(Pallet::<Test>::create_identity(RuntimeOrigin::signed(2)));
        assert_ok!(Pallet::<Test>::create_identity(RuntimeOrigin::signed(3)));
        assert_eq!(Pallet::<Test>::identity_number(), 3);
	});
}

#[test]
fn revoke_identity_works_and_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_ok!(Pallet::<Test>::create_identity(RuntimeOrigin::signed(10)));
        assert_ok!(Pallet::<Test>::revoke_identity(RuntimeOrigin::signed(10), 0));
        System::assert_last_event(Event::IdentityRevoked { identity_id: 0, owner: 10 }.into());
        assert!(Pallet::<Test>::identity_list(0).is_none());
	});
}

#[test]
fn revoke_identity_from_non_owner_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_ok!(Pallet::<Test>::create_identity(RuntimeOrigin::signed(10)));
        assert_noop!(Pallet::<Test>::revoke_identity(RuntimeOrigin::signed(20), 0), Error::<Test>::IdentityNotOwned);
	});
}

#[test]
fn revoke_nonexistent_identity_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_noop!(Pallet::<Test>::revoke_identity(RuntimeOrigin::signed(1), 99), Error::<Test>::IdentityNotOwned);
	});
}

#[test]
fn add_or_update_identity_trait_works_and_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        let account_id = 42;
        type MaxSize = <Test as pallet_identity::Config>::MaxSize;
        let key = BoundedVec::<u8, MaxSize>::try_from(b"name".to_vec()).unwrap();
        let value = BoundedVec::<u8, MaxSize>::try_from(b"Luke Skywalker".to_vec()).unwrap();
        assert_ok!(Pallet::<Test>::create_identity(RuntimeOrigin::signed(account_id)));
        assert_ok!(Pallet::<Test>::add_or_update_identity_trait(RuntimeOrigin::signed(account_id), 0, key.clone(), value.clone()));
        System::assert_last_event(Event::IdentityUpdated { identity_id: 0, owner: account_id }.into());
        assert_eq!(Pallet::<Test>::identity_trait_list(0, key.clone()), value);
	});
}

#[test]
fn add_or_update_identity_trait_non_owner_should_fail() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_ok!(Pallet::<Test>::create_identity(RuntimeOrigin::signed(10)));
        type MaxSize = <Test as pallet_identity::Config>::MaxSize;
        let key = BoundedVec::<u8, MaxSize>::try_from(b"name".to_vec()).unwrap();
        let value = BoundedVec::<u8, MaxSize>::try_from(b"Luke Skywalker".to_vec()).unwrap();
        assert_noop!(Pallet::<Test>::add_or_update_identity_trait(RuntimeOrigin::signed(20), 0, key, value), Error::<Test>::IdentityNotOwned);
	});
}

#[test]
fn remove_identity_trait_works_and_emits_event() {
	new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let account_id = 42;
        type MaxSize = <Test as pallet_identity::Config>::MaxSize;
        let key = BoundedVec::<u8, MaxSize>::try_from(b"name".to_vec()).unwrap();
        let value = BoundedVec::<u8, MaxSize>::try_from(b"Luke Skywalker".to_vec()).unwrap();
        assert_ok!(Pallet::<Test>::create_identity(RuntimeOrigin::signed(account_id)));
        assert_ok!(Pallet::<Test>::add_or_update_identity_trait(RuntimeOrigin::signed(account_id), 0, key.clone(), value.clone()));
        assert_ok!(Pallet::<Test>::remove_identity_trait(RuntimeOrigin::signed(account_id), 0, key.clone()));
        System::assert_last_event(Event::IdentityUpdated { identity_id: 0, owner: account_id }.into());
        assert_eq!(Pallet::<Test>::identity_trait_list(0, key), BoundedVec::<u8, <Test as pallet_identity::Config>::MaxSize>::default());
	});
}

#[test]
fn remove_identity_trait_non_owner_should_fail() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_ok!(Pallet::<Test>::create_identity(RuntimeOrigin::signed(10)));
        type MaxSize = <Test as pallet_identity::Config>::MaxSize;
        let key = BoundedVec::<u8, MaxSize>::try_from(b"name".to_vec()).unwrap();
        let value = BoundedVec::<u8, MaxSize>::try_from(b"Luke Skywalker".to_vec()).unwrap();
        assert_ok!(Pallet::<Test>::add_or_update_identity_trait(RuntimeOrigin::signed(10), 0, key.clone(), value));
        assert_noop!(Pallet::<Test>::remove_identity_trait(RuntimeOrigin::signed(20), 0, key), Error::<Test>::IdentityNotOwned);
    });
}

#[test]
fn defaults_are_zeroed() {
    new_test_ext().execute_with(|| {
        assert!(Pallet::<Test>::identity_list(0).is_none());
        assert_eq!(Pallet::<Test>::identity_number(), 0);
    });
}

#[test]
fn create_identity_storage_overflow() {
    new_test_ext().execute_with(|| {
        // Simulate storage overflow by setting identity_number to max value
        IdentityNumber::<Test>::put(u32::MAX);
        assert_noop!(Pallet::<Test>::create_identity(RuntimeOrigin::signed(1)), Error::<Test>::StorageOverflow);
    });
}

#[test]
fn add_trait_on_nonexistent_identity_should_fail() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        type MaxSize = <Test as pallet_identity::Config>::MaxSize;
        let key = BoundedVec::<u8, MaxSize>::try_from(b"name".to_vec()).unwrap();
        let value = BoundedVec::<u8, MaxSize>::try_from(b"Luke Skywalker".to_vec()).unwrap();
        assert_noop!(Pallet::<Test>::add_or_update_identity_trait(RuntimeOrigin::signed(1), 99, key, value), Error::<Test>::IdentityNotOwned);
	});
}

#[test]
fn remove_trait_on_nonexistent_identity_should_fail() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        type MaxSize = <Test as pallet_identity::Config>::MaxSize;
        let key = BoundedVec::<u8, MaxSize>::try_from(b"name".to_vec()).unwrap();
        assert_noop!(Pallet::<Test>::remove_identity_trait(RuntimeOrigin::signed(1), 99, key), Error::<Test>::IdentityNotOwned);
	});
}
