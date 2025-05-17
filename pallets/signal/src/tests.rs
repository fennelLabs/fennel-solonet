use crate::{mock::*, Error, Event, RatingSignalList, SignalParameterList};
use frame_support::{assert_ok, assert_noop};
use sp_core::ConstU32;
use frame_support::BoundedVec;
use frame_support::traits::Currency;

#[test]
fn set_signal_parameter_works_and_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        let name = BoundedVec::<u8, ConstU32<1024>>::try_from(b"PARAM".to_vec()).unwrap();
        assert_ok!(Signal::set_signal_parameter(RuntimeOrigin::signed(1), name.clone(), 42));
        // Storage check
        assert_eq!(SignalParameterList::<Test>::get(1, &name), 42);
        // Event check
        System::assert_last_event(Event::SignalParameterSet { who: 1 }.into());
	});
}

#[test]
fn send_rating_signal_works_and_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        Balances::make_free_balance_be(&1, 100u32.into());
        let target = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TARGET".to_vec()).unwrap();
        assert_ok!(Signal::send_rating_signal(RuntimeOrigin::signed(1), target.clone(), 5));
        // Storage check
        assert_eq!(RatingSignalList::<Test>::get(1, &target), 5);
        // Event check
        System::assert_has_event(Event::SignalLock { account: 1, amount: 10u32.into() }.into());
        System::assert_last_event(Event::RatingSignalSent { who: 1 }.into());
	});
}

#[test]
fn cannot_send_duplicate_rating_signal() {
	new_test_ext().execute_with(|| {
        System::set_block_number(1);
        Balances::make_free_balance_be(&1, 100u32.into());
        let target = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TARGET".to_vec()).unwrap();
        assert_ok!(Signal::send_rating_signal(RuntimeOrigin::signed(1), target.clone(), 5));
        assert_noop!(Signal::send_rating_signal(RuntimeOrigin::signed(1), target.clone(), 7), Error::<Test>::RatingSignalAlreadyExists);
    });
}

#[test]
fn cannot_send_rating_signal_with_insufficient_balance() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        let target = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TARGET".to_vec()).unwrap();
        assert_noop!(Signal::send_rating_signal(RuntimeOrigin::signed(1), target, 5), Error::<Test>::InsufficientBalance);
	});
}

#[test]
fn update_rating_signal_works_and_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        Balances::make_free_balance_be(&1, 100u32.into());
        let target = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TARGET".to_vec()).unwrap();
        assert_ok!(Signal::send_rating_signal(RuntimeOrigin::signed(1), target.clone(), 5));
        assert_ok!(Signal::update_rating_signal(RuntimeOrigin::signed(1), target.clone(), 9));
        // Storage check
        assert_eq!(RatingSignalList::<Test>::get(1, &target), 9);
        // Event check
        System::assert_has_event(Event::SignalLockExtended { account: 1, amount: 10u32.into() }.into());
        System::assert_last_event(Event::RatingSignalUpdated { who: 1 }.into());
	});
}

#[test]
fn cannot_update_nonexistent_rating_signal() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        Balances::make_free_balance_be(&1, 100u32.into());
        let target = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TARGET".to_vec()).unwrap();
        assert_noop!(Signal::update_rating_signal(RuntimeOrigin::signed(1), target, 9), Error::<Test>::RatingSignalDoesNotExist);
	});
}

#[test]
fn revoke_rating_signal_works_and_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        Balances::make_free_balance_be(&1, 100u32.into());
        let target = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TARGET".to_vec()).unwrap();
        assert_ok!(Signal::send_rating_signal(RuntimeOrigin::signed(1), target.clone(), 5));
        assert_ok!(Signal::revoke_rating_signal(RuntimeOrigin::signed(1), target.clone()));
        // Storage check
        assert_eq!(RatingSignalList::<Test>::get(1, &target), 0);
        // Event check
        System::assert_has_event(Event::SignalUnlock { account: 1 }.into());
        System::assert_last_event(Event::RatingSignalRevoked { who: 1 }.into());
	});
}

#[test]
fn cannot_revoke_nonexistent_rating_signal() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
        let target = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TARGET".to_vec()).unwrap();
        assert_noop!(Signal::revoke_rating_signal(RuntimeOrigin::signed(1), target), Error::<Test>::RatingSignalDoesNotExist);
    });
}

#[test]
fn send_signal_works_and_emits_event() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let signal = BoundedVec::<u8, ConstU32<1024>>::try_from(b"SIGNAL".to_vec()).unwrap();
        assert_ok!(Signal::send_signal(RuntimeOrigin::signed(1), signal.clone()));
        System::assert_last_event(Event::SignalSent { signal, who: 1 }.into());
    });
}

#[test]
fn send_service_signal_works_and_emits_event() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let service_identifier = BoundedVec::<u8, ConstU32<1024>>::try_from(b"SERVICE".to_vec()).unwrap();
        let url = BoundedVec::<u8, ConstU32<1024>>::try_from(b"URL".to_vec()).unwrap();
        assert_ok!(Signal::send_service_signal(RuntimeOrigin::signed(1), service_identifier.clone(), url.clone()));
        System::assert_last_event(Event::ServiceSignalSent { service_identifier, url, who: 1 }.into());
	});
}
