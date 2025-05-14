//! Tests for the validator-manager pallet

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::OnInitialize;
use sp_runtime::testing::UintAuthorityId;
use crate::mock::{
    System, Session, ValidatorManager, RuntimeOrigin, Test, ValidatorId,
};

fn validator_keys(c: &[u64]) -> Vec<ValidatorId> {
    c.iter().copied().map(ValidatorId).collect()
}

#[test]
fn initial_validators_should_be_set() {
    new_test_ext().execute_with(|| {
        // Start at session 1 and advance to session 2 to apply initial validators
        Session::on_initialize(1);
        assert_eq!(Session::validators(), validator_keys(&[1, 2, 3]));
    });
}

#[test]
fn add_validators_should_work() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        Session::on_initialize(1);
        assert_eq!(Session::validators(), validator_keys(&[1, 2, 3]));
        // Ensure account 4 exists
        let _ = System::inc_consumers(&ValidatorId(4));
        System::account_nonce(ValidatorId(4));
        // Set session keys for validator 4 before registering
        assert_ok!(Session::set_keys(
            RuntimeOrigin::signed(ValidatorId(4).into()),
            UintAuthorityId(4),
            Vec::new(),
        ));
        // Register a new validator (privileged origin)
        assert_ok!(ValidatorManager::register_validators(
            RuntimeOrigin::root(),
            validator_keys(&[4])
        ));
        // Check that the validator is in the queue
        assert_eq!(ValidatorManager::validators_to_add(), validator_keys(&[4]));
        // Trigger five more sessions to enact the change (two full rotations after registration)
        Session::on_initialize(2);
        Session::on_initialize(3);
        Session::on_initialize(4);
        Session::on_initialize(5);
        Session::on_initialize(6);
        // Validators should now include the new one
        assert_eq!(Session::validators(), validator_keys(&[1, 2, 3, 4]));
        // Check the event was emitted
        System::assert_has_event(
            Event::ValidatorsRegistered { validators: validator_keys(&[4]) }.into(),
        );
    });
}

#[test]
fn cannot_add_duplicate_validator() {
    new_test_ext().execute_with(|| {
        assert_ok!(ValidatorManager::register_validators(
            RuntimeOrigin::root(),
            validator_keys(&[4])
        ));
        // Attempt to add it again should fail
        assert_noop!(
            ValidatorManager::register_validators(
                RuntimeOrigin::root(),
                validator_keys(&[4])
            ),
            Error::<Test>::ValidatorAlreadyAdded
        );
    });
}

#[test]
fn remove_validator_should_work() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        Session::on_initialize(1);
        assert_eq!(Session::validators(), validator_keys(&[1, 2, 3]));
        // Remove validator 2 (privileged origin)
        assert_ok!(ValidatorManager::remove_validator(
            RuntimeOrigin::root(),
            ValidatorId(2)
        ));
        // Check that the validator is in the removal queue
        assert_eq!(ValidatorManager::validators_to_remove(), validator_keys(&[2]));
        // Trigger three more sessions to enact the change
        Session::on_initialize(2);
        Session::on_initialize(3);
        Session::on_initialize(4);
        // Validators should no longer include the removed one
        assert_eq!(Session::validators(), validator_keys(&[1, 3]));
        // Check the event was emitted
        System::assert_has_event(
            Event::ValidatorRemoved { validator: ValidatorId(2) }.into(),
        );
    });
}

#[test]
fn cannot_remove_nonexistent_validator() {
    new_test_ext().execute_with(|| {
        Session::on_initialize(1);
        // Attempt to remove a non-existent validator
        assert_noop!(
            ValidatorManager::remove_validator(
                RuntimeOrigin::root(),
                ValidatorId(99)
            ),
            Error::<Test>::NotValidator
        );
    });
}

#[test]
fn cannot_remove_below_min_validators() {
    new_test_ext().execute_with(|| {
        Session::on_initialize(1);
        assert_eq!(Session::validators(), validator_keys(&[1, 2, 3]));
        // Remove validator 2
        assert_ok!(ValidatorManager::remove_validator(
            RuntimeOrigin::root(),
            ValidatorId(2)
        ));
        // Remove validator 3 (should fail due to min authorities)
        assert_noop!(
            ValidatorManager::remove_validator(
                RuntimeOrigin::root(),
                ValidatorId(3)
            ),
            Error::<Test>::TooFewValidators
        );
    });
}

#[test]
fn unauthorized_origin_cannot_add_validators() {
    new_test_ext().execute_with(|| {
        // Use an unauthorized account (not root)
        assert_noop!(
            ValidatorManager::register_validators(
                RuntimeOrigin::signed(ValidatorId(2).into()),
                validator_keys(&[4])
            ),
            frame_support::error::BadOrigin
        );
    });
}

#[test]
fn unauthorized_origin_cannot_remove_validators() {
    new_test_ext().execute_with(|| {
        Session::on_initialize(1);
        // Use an unauthorized account (not root)
        assert_noop!(
            ValidatorManager::remove_validator(
                RuntimeOrigin::signed(ValidatorId(2).into()),
                ValidatorId(3)
            ),
            frame_support::error::BadOrigin
        );
    });
}