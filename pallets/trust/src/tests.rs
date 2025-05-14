use crate::{mock::*, Error, Event, TrustParameterList, TrustIssuance, TrustRevocation, TrustRequestList};
use frame_support::{assert_ok, assert_noop};
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

type TrustModule = crate::Pallet<Test>;

#[test]
fn test_set_trust_parameter() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        let param = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TEST".to_vec()).unwrap();
        assert_ok!(TrustModule::set_trust_parameter(RuntimeOrigin::signed(1), param.clone(), 42));
        assert_eq!(TrustParameterList::<Test>::get(1, &param), 42);
        System::assert_last_event(Event::TrustParameterSet { who: 1 }.into());
	});
}

#[test]
fn test_issue_trust() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 2));
        assert_eq!(TrustIssuance::<Test>::get(1, 2), Some(0));
		assert_eq!(TrustModule::get_current_trust_count(), 1);
        System::assert_last_event(Event::TrustIssued { issuer: 1, target: 2 }.into());
	});
}

#[test]
fn test_issue_trust_error() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 2));
        assert_noop!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 2), Error::<Test>::TrustExists);
	});
}

#[test]
fn test_remove_trust() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 2));
        assert_ok!(TrustModule::remove_trust(RuntimeOrigin::signed(1), 2));
        assert_eq!(TrustIssuance::<Test>::get(1, 2), None); // Should be removed
        assert_eq!(TrustModule::get_current_trust_count(), 0);
        System::assert_last_event(Event::TrustIssuanceRemoved { issuer: 1, target: 2 }.into());
	});
}

#[test]
fn test_remove_trust_error() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_noop!(TrustModule::remove_trust(RuntimeOrigin::signed(1), 2), Error::<Test>::TrustNotFound);
	});
}

#[test]
fn test_request_and_cancel_trust() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_ok!(TrustModule::request_trust(RuntimeOrigin::signed(1), 2));
        assert_eq!(TrustRequestList::<Test>::get(1, 2), Some(0));
		assert_eq!(TrustModule::get_current_trust_requests(), 1);
        System::assert_last_event(Event::TrustRequest { requester: 1, target: 2 }.into());
        assert_ok!(TrustModule::cancel_trust_request(RuntimeOrigin::signed(1), 2));
		assert_eq!(TrustModule::get_current_trust_requests(), 0);
        System::assert_last_event(Event::TrustRequestRemoved { requester: 1, target: 2 }.into());
	});
}

#[test]
fn test_cancel_trust_request_error() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_noop!(TrustModule::cancel_trust_request(RuntimeOrigin::signed(1), 2), Error::<Test>::TrustRequestNotFound);
	});
}

#[test]
fn test_revoke_and_remove_revoked_trust() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_ok!(TrustModule::revoke_trust(RuntimeOrigin::signed(1), 2));
        assert_eq!(TrustRevocation::<Test>::get(1, 2), Some(0));
        System::assert_last_event(Event::TrustRevoked { issuer: 1, target: 2 }.into());
        assert_ok!(TrustModule::remove_revoked_trust(RuntimeOrigin::signed(1), 2));
        System::assert_last_event(Event::TrustRevocationRemoved { issuer: 1, target: 2 }.into());
	});
}

#[test]
fn test_remove_revoked_trust_error() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        assert_noop!(TrustModule::remove_revoked_trust(RuntimeOrigin::signed(1), 2), Error::<Test>::TrustRevocationNotFound);
	});
}
