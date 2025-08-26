//! Benchmarking setup for pallet-validator-manager

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_benchmarking::account;
use codec::Decode;

fn validator_id<T: Config>(seed: u32) -> T::ValidatorId {
    let account: T::AccountId = account("validator", seed, 0);
    T::ValidatorOf::convert(account).expect("convert always returns Some for mock/test")
}

#[benchmarks]
mod benchmarks {
    use super::*;
    use frame_system::RawOrigin;

    #[benchmark]
    fn register_validators<T: Config>(c: Linear<1, 10>) {
        ValidatorsToAdd::<T>::kill(); // Clear storage before running
        let validators: Vec<T::ValidatorId> = (0..c).map(|i| validator_id::<T>(i as u32)).collect();
        
        // Set session keys for all validators before registering
        // This is required now that we check for keys existence
        for (i, validator) in validators.iter().enumerate() {
            let _account: T::AccountId = account("validator", i as u32, 0);
            // Create dummy session keys for benchmarking
            let keys = T::Keys::decode(&mut sp_runtime::traits::TrailingZeroInput::zeroes())
                .expect("Failed to decode zero keys");
            <pallet_session::NextKeys<T>>::insert(&validator, &keys);
        }
        
        #[extrinsic_call]
        Pallet::<T>::register_validators(RawOrigin::Root, validators.clone());
        assert_eq!(ValidatorsToAdd::<T>::get().len(), c as usize);
    }

    #[benchmark]
    fn remove_validator<T: Config>() {
        // Ensure validator 1 is in the set before removal
        let validator_id: T::ValidatorId = validator_id::<T>(1);
        let mut validators = Session::<T>::validators();
        if !validators.contains(&validator_id) {
            validators.push(validator_id.clone());
            // This assumes a helper exists to set validators in mock/test
            <pallet_session::Validators<T>>::put(validators.clone());
        }
        #[extrinsic_call]
        Pallet::<T>::remove_validator(RawOrigin::Root, validator_id);
        assert_eq!(ValidatorsToRemove::<T>::get().len(), 1);
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}

#[cfg(test)]
mod tests {
    // Remove unused imports from test module
    // use super::*;
    // use crate::mock::{new_test_ext, Test};
    // use frame_support::assert_ok;

    // #[test]
    // fn test_benchmarks() {
    //     new_test_ext().execute_with(|| {
    //         assert_ok!(test_benchmark_register_validators::<Test>());
    //         assert_ok!(test_benchmark_remove_validator::<Test>());
    //     });
    // }
}