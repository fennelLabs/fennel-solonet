use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

#[test]
fn create_submission_entry_works_and_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let _ = Balances::deposit_creating(&1, 100);
        let resource = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TEST".to_vec()).unwrap();
        assert_ok!(Infostratus::create_submission_entry(RuntimeOrigin::signed(1), resource.clone()));
		System::assert_last_event(
            crate::Event::SubmissionSent { who: 1, resource_location: resource }.into()
		);
	});
}

#[test]
fn cannot_create_duplicate_submission() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let _ = Balances::deposit_creating(&1, 100);
        let resource = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TEST".to_vec()).unwrap();
        assert_ok!(Infostratus::create_submission_entry(RuntimeOrigin::signed(1), resource.clone()));
		assert_noop!(
            Infostratus::create_submission_entry(RuntimeOrigin::signed(1), resource.clone()),
			Error::<Test>::SubmissionExists
		);
	});
}

#[test]
fn request_submission_assignment_works_and_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let _ = Balances::deposit_creating(&1, 100);
		let _ = Balances::deposit_creating(&2, 100);
        let resource = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TEST".to_vec()).unwrap();
        assert_ok!(Infostratus::create_submission_entry(RuntimeOrigin::signed(1), resource.clone()));
        assert_ok!(Infostratus::request_submission_assignment(RuntimeOrigin::signed(2), 1, resource.clone()));
		System::assert_last_event(
            crate::Event::SubmissionAssigned { resource_location: resource, who: 2 }.into()
		);
	});
}

#[test]
fn cannot_assign_nonexistent_submission() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let _ = Balances::deposit_creating(&1, 100);
		let _ = Balances::deposit_creating(&2, 100);
        let resource = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TEST".to_vec()).unwrap();
		assert_noop!(
            Infostratus::request_submission_assignment(RuntimeOrigin::signed(2), 1, resource),
			Error::<Test>::SubmissionDoesNotExist
		);
	});
}

#[test]
fn cannot_assign_already_assigned_submission() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let _ = Balances::deposit_creating(&1, 100);
		let _ = Balances::deposit_creating(&2, 100);
		let _ = Balances::deposit_creating(&3, 100);
        let resource = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TEST".to_vec()).unwrap();
        assert_ok!(Infostratus::create_submission_entry(RuntimeOrigin::signed(1), resource.clone()));
        assert_ok!(Infostratus::request_submission_assignment(RuntimeOrigin::signed(2), 1, resource.clone()));
		assert_noop!(
            Infostratus::request_submission_assignment(RuntimeOrigin::signed(3), 1, resource.clone()),
			Error::<Test>::SubmissionAlreadyAssigned
		);
	});
}

#[test]
fn cannot_assign_own_submission() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let _ = Balances::deposit_creating(&1, 100);
        let resource = BoundedVec::<u8, ConstU32<1024>>::try_from(b"TEST".to_vec()).unwrap();
        assert_ok!(Infostratus::create_submission_entry(RuntimeOrigin::signed(1), resource.clone()));
		assert_noop!(
            Infostratus::request_submission_assignment(RuntimeOrigin::signed(1), 1, resource),
			Error::<Test>::CannotAssignOwnSubmission
		);
	});
}
