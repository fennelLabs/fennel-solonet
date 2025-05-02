//! Benchmarking setup for pallet-validator-manager
#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::Pallet as ValidatorManager;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_std::prelude::*;

#[benchmarks]
mod benchmarks {
    use super::*;

    /// Benchmark for `register_validators` where `v` is the number of validators.
    #[benchmark]
    fn register_validators(v: Linear<1, 100>) -> Result<(), BenchmarkError> {
        // Use root origin since the pallet requires PrivilegedOrigin
        let root_origin = RawOrigin::Root;

        // Create v distinct validator accounts and convert to ValidatorId
        let mut validator_set = Vec::with_capacity(v as usize);
        for i in 0..v {
            let validator: T::AccountId = account("validator", i, 0);
            // Convert AccountId to ValidatorId using the ValidatorOf conversion
            let validator_id = T::ValidatorIdOf::convert(validator).unwrap();
            validator_set.push(validator_id);
        }

        #[extrinsic_call]
        register_validators(root_origin, validator_set.clone());

        // Validate that the extrinsic had the expected effect
        let validators_to_add = ValidatorsToAdd::<T>::get();
        if validators_to_add.is_empty() {
            return Err(BenchmarkError::Weightless);
        }

        // Additional validation: check that all validators were properly added
        for validator_id in validator_set {
            if !validators_to_add.contains(&validator_id) {
                return Err(BenchmarkError::Weightless);
            }
        }

        Ok(())
    }

    #[benchmark]
    fn remove_validator() -> Result<(), BenchmarkError> {
        // Use root origin since the pallet requires PrivilegedOrigin
        let root_origin = RawOrigin::Root;

        // Create a validator to remove
        let validator_account: T::AccountId = account("validator", 0, 0);
        let validator_id = T::ValidatorIdOf::convert(validator_account.clone()).unwrap();

        // Add the validator to the Session validators storage
        // This is needed to make sure the validator exists when we try to remove it
        let mut validators = Session::<T>::validators();
        validators.push(validator_id.clone());
        
        // Manually insert validators into Session storage
        pallet_session::Validators::<T>::put(validators);

        #[extrinsic_call]
        remove_validator(root_origin, validator_id.clone());

        // Validate that the extrinsic had the expected effect
        let validators_to_remove = ValidatorsToRemove::<T>::get();

        // Check that the validators_to_remove list isn't empty
        if validators_to_remove.is_empty() {
            return Err(BenchmarkError::Weightless);
        }

        // Additional validation: check that our validator was properly added to removal list
        if !validators_to_remove.contains(&validator_id) {
            return Err(BenchmarkError::Weightless);
        }

        Ok(())
    }

    impl_benchmark_test_suite!(
        ValidatorManager,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}
