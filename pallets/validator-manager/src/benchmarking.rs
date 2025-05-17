//! Benchmarking setup for pallet-validator-manager

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;

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