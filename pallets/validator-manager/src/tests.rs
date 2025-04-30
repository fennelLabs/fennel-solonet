#![cfg(test)]

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

// Helper function to properly process validator changes in tests
fn process_validator_changes() {
    // Manually process validator additions and removals using the validator manager
    let new_validators = ValidatorManager::new_session(1);

    // If we have a valid new set of validators, update the Session pallet's validators
    if let Some(validators) = new_validators {
        // In a real chain, this happens automatically, but we need to manually
        // update the validators in our tests
        pallet_session::CurrentIndex::<Test>::put(1);
        pallet_session::Validators::<Test>::put(validators);
    }
}

#[test]
fn register_validators_works() {
    new_test_ext().execute_with(|| {
        // Get the initial validators
        let initial_validators = Session::validators();
        assert_eq!(initial_validators, vec![1, 2, 3]);

        // Register a new validator
        assert_ok!(ValidatorManager::register_validators(
            RuntimeOrigin::root(),
            vec![4]
        ));

        // Check if the validator was added to the list
        assert_eq!(ValidatorManager::validators_to_add(), vec![4]);

        // Process changes
        process_validator_changes();

        // The validator should now be in the active set
        let new_validators = Session::validators();
        assert_eq!(new_validators, vec![1, 2, 3, 4]);
        assert!(new_validators.contains(&4));
    });
}

#[test]
fn register_validators_emits_event() {
    new_test_ext().execute_with(|| {
        // Set block number to ensure events are recorded
        System::set_block_number(1);

        // Register a new validator
        assert_ok!(ValidatorManager::register_validators(
            RuntimeOrigin::root(),
            vec![4]
        ));

        // Check that the ValidatorsRegistered event was emitted
        System::assert_last_event(
            Event::ValidatorsRegistered {
                validators: vec![4],
            }
            .into(),
        );
    });
}

#[test]
fn register_validators_fails_for_already_added() {
    new_test_ext().execute_with(|| {
        // Add validator 4
        assert_ok!(ValidatorManager::register_validators(
            RuntimeOrigin::root(),
            vec![4]
        ));

        // Try to add validator 4 again
        assert_noop!(
            ValidatorManager::register_validators(RuntimeOrigin::root(), vec![4]),
            Error::<Test>::ValidatorAlreadyAdded
        );
    });
}

#[test]
fn register_validators_fails_for_non_root() {
    new_test_ext().execute_with(|| {
        // Try to add validator from non-root origin
        assert_noop!(
            ValidatorManager::register_validators(RuntimeOrigin::signed(1), vec![4]),
            BadOrigin
        );
    });
}

#[test]
fn remove_validator_works() {
    new_test_ext().execute_with(|| {
        // Verify validator 1 is in the initial set
        let initial_validators = Session::validators();
        assert_eq!(initial_validators, vec![1, 2, 3]);
        assert!(initial_validators.contains(&1));

        // Remove validator 1
        assert_ok!(ValidatorManager::remove_validator(RuntimeOrigin::root(), 1));

        // Check if the validator was added to the removal list
        assert_eq!(ValidatorManager::validators_to_remove(), vec![1]);

        // Process changes
        process_validator_changes();

        // The validator should now be removed from the active set
        let new_validators = Session::validators();
        assert_eq!(new_validators, vec![3, 2]); // Note: swap_remove changes order
        assert!(!new_validators.contains(&1));
    });
}

#[test]
fn remove_validator_emits_event() {
    new_test_ext().execute_with(|| {
        // Set block number to ensure events are recorded
        System::set_block_number(1);

        // Remove validator 1
        assert_ok!(ValidatorManager::remove_validator(RuntimeOrigin::root(), 1));

        // Check that the ValidatorRemoved event was emitted
        System::assert_last_event(Event::ValidatorRemoved { validator: 1 }.into());
    });
}

#[test]
fn remove_validator_fails_for_non_validator() {
    new_test_ext().execute_with(|| {
        // Try to remove validator 10 which is not in the set
        assert_noop!(
            ValidatorManager::remove_validator(RuntimeOrigin::root(), 10),
            Error::<Test>::NotValidator
        );
    });
}

#[test]
fn remove_validator_fails_for_non_root() {
    new_test_ext().execute_with(|| {
        // Try to remove validator from non-root origin
        assert_noop!(
            ValidatorManager::remove_validator(RuntimeOrigin::signed(1), 1),
            BadOrigin
        );
    });
}

#[test]
fn maintain_min_validators() {
    new_test_ext().execute_with(|| {
        // Initial set is [1, 2, 3]
        let initial_validators = Session::validators();
        assert_eq!(initial_validators, vec![1, 2, 3]);

        // Remove validators 2 and 3
        assert_ok!(ValidatorManager::remove_validator(RuntimeOrigin::root(), 2));
        assert_ok!(ValidatorManager::remove_validator(RuntimeOrigin::root(), 3));

        // Process changes
        process_validator_changes();

        // Verify we still have validator 1 in the active set
        let new_validators = Session::validators();
        assert_eq!(new_validators, vec![1]);

        // Now try to remove the last validator
        assert_ok!(ValidatorManager::remove_validator(RuntimeOrigin::root(), 1));

        // Directly get the new session result from ValidatorManager
        // This should return None since we'd drop below MinAuthorities = 1
        let result = ValidatorManager::new_session(2);
        assert!(result.is_none());

        // Since it returned None, the validators should remain the same
        assert_eq!(Session::validators(), vec![1]);
    });
}

#[test]
fn new_session_maintains_validators() {
    new_test_ext().execute_with(|| {
        // Set up some changes
        System::set_block_number(1);

        // Register a new validator
        assert_ok!(ValidatorManager::register_validators(
            RuntimeOrigin::root(),
            vec![4]
        ));
        assert_ok!(ValidatorManager::remove_validator(RuntimeOrigin::root(), 2));

        // Verify correct events were emitted
        System::assert_has_event(
            Event::ValidatorsRegistered {
                validators: vec![4],
            }
            .into(),
        );
        System::assert_has_event(Event::ValidatorRemoved { validator: 2 }.into());

        // Apply changes and verify the updated validator set
        process_validator_changes();
        let updated_validators = Session::validators();
        assert!(updated_validators.contains(&4));
        assert!(!updated_validators.contains(&2));
    });
}
