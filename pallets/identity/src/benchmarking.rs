//! FRAME benchmarking for pallet-identity.

#![cfg(feature = "runtime-benchmarks")]

use crate::{Config};
use frame_benchmarking::v2::*;
use scale_info::prelude::vec;

#[benchmarks(
    where
        T: Config<MaxSize = frame_support::traits::ConstU32<1024>>,
)]
mod benchmarks {
	use super::*;
    use crate::{Call, Pallet};
    use frame_system::RawOrigin;
    use frame_support::BoundedVec;
    use frame_benchmarking::{BenchmarkError, whitelisted_caller};

	#[benchmark]
	fn create_identity() -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
        Pallet::<T>::create_identity(RawOrigin::Signed(who.clone()));
        let last = Pallet::<T>::identity_number().saturating_sub(1);
        assert_eq!(Pallet::<T>::identity_list(last), Some(who));
		Ok(())
	}

	#[benchmark]
	fn revoke_identity() -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        Pallet::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        let last = Pallet::<T>::identity_number().saturating_sub(1);
		#[extrinsic_call]
        Pallet::<T>::revoke_identity(RawOrigin::Signed(who.clone()), last);
        // Check that the identity no longer exists in the IdentityList.
        assert!(Pallet::<T>::identity_list(last).is_none());
		Ok(())
	}

	#[benchmark]
    fn add_or_update_identity_trait(l: Linear<1, 1024>) -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        Pallet::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        let last = Pallet::<T>::identity_number().saturating_sub(1);
        let key: BoundedVec<u8, T::MaxSize> = vec![0u8; l as usize].try_into().unwrap();
        let val: BoundedVec<u8, T::MaxSize> = vec![1u8; l as usize].try_into().unwrap();
		#[extrinsic_call]
        Pallet::<T>::add_or_update_identity_trait(RawOrigin::Signed(who.clone()), last, key.clone(), val.clone());
        // Check that the identity no longer exists in the IdentityList.
        assert_eq!(Pallet::<T>::identity_trait_list(last, key), val);
		Ok(())
	}

	#[benchmark]
    fn remove_identity_trait(l: Linear<1, 1024>) -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        Pallet::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        let last = Pallet::<T>::identity_number().saturating_sub(1);
        let key: BoundedVec<u8, T::MaxSize> = vec![0u8; l as usize].try_into().unwrap();
        let val: BoundedVec<u8, T::MaxSize> = vec![1u8; l as usize].try_into().unwrap();
        Pallet::<T>::add_or_update_identity_trait(
            RawOrigin::Signed(who.clone()).into(),
            last,
            key.clone(),
            val,
		)?;

		#[extrinsic_call]
        Pallet::<T>::remove_identity_trait(RawOrigin::Signed(who.clone()), last, key.clone());
        assert_eq!(Pallet::<T>::identity_trait_list(last, key), BoundedVec::<u8, T::MaxSize>::default());
		Ok(())
	}

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
