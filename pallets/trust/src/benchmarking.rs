//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Trust;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::BoundedVec;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_trust_parameter() -> Result<(), BenchmarkError> {
		let param = BoundedVec::<u8, <T as pallet::Config>::MaxTrustParameterSize>::try_from(b"TEST".to_vec()).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), param.clone(), 42);
        assert_eq!(TrustParameterList::<T>::get(caller.clone(), &param), 42);
        frame_system::Pallet::<T>::assert_last_event(
            <T as pallet::Config>::RuntimeEvent::from(Event::<T>::TrustParameterSet { who: caller }).into()
        );
		Ok(())
	}

	#[benchmark]
	fn issue_trust() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();
        let target: T::AccountId = benchmark_account("target", 0, 0);
		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), target.clone());
		assert_eq!(CurrentIssued::<T>::get(), 1);
        assert_eq!(TrustIssuance::<T>::get(caller.clone(), target.clone()), Some(0));
        frame_system::Pallet::<T>::assert_last_event(
            <T as pallet::Config>::RuntimeEvent::from(Event::<T>::TrustIssued { issuer: caller, target }).into()
        );
		Ok(())
	}

	#[benchmark]
	fn issue_trust_repeatedly(m: Linear<0, 1000>) -> Result<(), BenchmarkError> {
		for i in 0..m {
			let target: T::AccountId = benchmark_account("target", i, 0);
			let caller: T::AccountId = benchmark_account("caller", i, 0);
			Trust::<T>::issue_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
		}

		let target: T::AccountId = whitelisted_caller();
		let caller: T::AccountId = whitelisted_caller();

		#[extrinsic_call]
		issue_trust(RawOrigin::Signed(caller.clone()), target.clone());

		assert_eq!(CurrentIssued::<T>::get(), m + 1);
		assert!(TrustIssuance::<T>::contains_key(caller.clone(), target.clone()));

		Ok(())
	}

	#[benchmark]
	fn revoke_trust() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();
        let target: T::AccountId = benchmark_account("target", 0, 0);
		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), target.clone());
		assert_eq!(CurrentRevoked::<T>::get(), 1);
        assert_eq!(TrustRevocation::<T>::get(caller.clone(), target.clone()), Some(0));
        frame_system::Pallet::<T>::assert_last_event(
            <T as pallet::Config>::RuntimeEvent::from(Event::<T>::TrustRevoked { issuer: caller, target }).into()
        );
		Ok(())
	}

	#[benchmark]
	fn revoke_trust_from_heavy_storage(m: Linear<0, 100_000>) -> Result<(), BenchmarkError> {
		for i in 0..m {
			let target: T::AccountId = benchmark_account("target", i, 0);
			let caller: T::AccountId = benchmark_account("caller", i, 0);
			Trust::<T>::issue_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
		}

		for i in 0..m {
			let target: T::AccountId = benchmark_account("target", 100_000 + i, 0);
			let caller: T::AccountId = benchmark_account("caller", 100_000 + i, 0);
			Trust::<T>::revoke_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
		}

		let target: T::AccountId = whitelisted_caller();
		let caller: T::AccountId = whitelisted_caller();

		#[extrinsic_call]
		revoke_trust(RawOrigin::Signed(caller.clone()), target.clone());

		assert_eq!(CurrentRevoked::<T>::get(), m + 1);
		assert!(TrustRevocation::<T>::contains_key(caller.clone(), target.clone()));

		Ok(())
	}

	#[benchmark]
	fn remove_trust() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();
        let target: T::AccountId = benchmark_account("target", 0, 0);
		Trust::<T>::issue_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), target.clone());

		assert_eq!(CurrentIssued::<T>::get(), 0);
        assert_eq!(TrustIssuance::<T>::get(caller.clone(), target.clone()), None);
        frame_system::Pallet::<T>::assert_last_event(
            <T as pallet::Config>::RuntimeEvent::from(Event::<T>::TrustIssuanceRemoved { issuer: caller, target }).into()
        );
		Ok(())
	}

	#[benchmark]
	fn remove_trust_from_heavy_storage(m: Linear<0, 1000>) -> Result<(), BenchmarkError> {
		for i in 0..m {
			let target: T::AccountId = benchmark_account("target", i, 0);
			let caller: T::AccountId = benchmark_account("caller", i, 0);
			Trust::<T>::issue_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
		}

		let target: T::AccountId = whitelisted_caller();
		let caller: T::AccountId = whitelisted_caller();
		Trust::<T>::issue_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;

		#[extrinsic_call]
		remove_trust(RawOrigin::Signed(caller.clone()), target.clone());

		assert_eq!(CurrentIssued::<T>::get(), m);
		assert!(!TrustIssuance::<T>::contains_key(caller.clone(), target.clone()));

		Ok(())
	}

	#[benchmark]
	fn request_trust() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();
        let target: T::AccountId = benchmark_account("target", 0, 0);
		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), target.clone());

		assert_eq!(CurrentRequests::<T>::get(), 1);
        assert_eq!(TrustRequestList::<T>::get(caller.clone(), target.clone()), Some(0));
        frame_system::Pallet::<T>::assert_last_event(
            <T as pallet::Config>::RuntimeEvent::from(Event::<T>::TrustRequest { requester: caller, target }).into()
        );
		Ok(())
	}

	#[benchmark]
	fn request_trust_repeatedly(m: Linear<0, 1000>) -> Result<(), BenchmarkError> {
		for i in 0..m {
			let target: T::AccountId = benchmark_account("target", i, 0);
			let caller: T::AccountId = benchmark_account("caller", i, 0);
			Trust::<T>::request_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
		}

		let target: T::AccountId = whitelisted_caller();
		let caller: T::AccountId = whitelisted_caller();

		#[extrinsic_call]
		request_trust(RawOrigin::Signed(caller.clone()), target.clone());

		assert_eq!(CurrentRequests::<T>::get(), m + 1);
		assert!(TrustRequestList::<T>::contains_key(caller.clone(), target.clone()));

		Ok(())
	}

	#[benchmark]
	fn remove_revoked_trust() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();
        let target: T::AccountId = benchmark_account("target", 0, 0);
		Trust::<T>::revoke_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), target.clone());

		assert_eq!(CurrentRevoked::<T>::get(), 0);
        assert_eq!(TrustRevocation::<T>::get(caller.clone(), target.clone()), None);
        frame_system::Pallet::<T>::assert_last_event(
            <T as pallet::Config>::RuntimeEvent::from(Event::<T>::TrustRevocationRemoved { issuer: caller, target }).into()
        );
		Ok(())
	}

	#[benchmark]
	fn remove_revoked_trust_heavy_storage(m: Linear<0, 1000>) -> Result<(), BenchmarkError> {
		for i in 0..m {
			let target: T::AccountId = benchmark_account("target", i, 0);
			let caller: T::AccountId = benchmark_account("caller", i, 0);
			Trust::<T>::revoke_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
		}

		let target: T::AccountId = whitelisted_caller();
		let caller: T::AccountId = whitelisted_caller();

		Trust::<T>::revoke_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;

		#[extrinsic_call]
		remove_revoked_trust(RawOrigin::Signed(caller.clone()), target.clone());

		assert_eq!(CurrentRevoked::<T>::get(), m);
		assert!(!TrustRevocation::<T>::contains_key(caller.clone(), target.clone()));

		Ok(())
	}

	#[benchmark]
	fn cancel_trust_request() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();
        let target: T::AccountId = benchmark_account("target", 0, 0);
		Trust::<T>::request_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), target.clone());

		assert_eq!(CurrentRequests::<T>::get(), 0);
        assert_eq!(TrustRequestList::<T>::get(caller.clone(), target.clone()), None);
        frame_system::Pallet::<T>::assert_last_event(
            <T as pallet::Config>::RuntimeEvent::from(Event::<T>::TrustRequestRemoved { requester: caller, target }).into()
        );
		Ok(())
	}

	#[benchmark]
	fn cancel_trust_request_heavy_storage(m: Linear<0, 1000>) -> Result<(), BenchmarkError> {
		for i in 0..m {
			let target: T::AccountId = benchmark_account("target", i, 0);
			let caller: T::AccountId = benchmark_account("caller", i, 0);
			if i != 14 {
			Trust::<T>::request_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
		}
		}
		let target: T::AccountId = benchmark_account("target", 14, 0);
		let caller: T::AccountId = benchmark_account("caller", 14, 0);
		Trust::<T>::request_trust(RawOrigin::Signed(caller.clone()).into(), target.clone())?;
		#[extrinsic_call]
		cancel_trust_request(RawOrigin::Signed(caller.clone()), target.clone());
		assert_eq!(CurrentRequests::<T>::get(), m.saturating_sub(1));
		assert!(!TrustRequestList::<T>::contains_key(caller.clone(), target.clone()));

		Ok(())
	}

	impl_benchmark_test_suite!(Trust, crate::mock::new_test_ext(), crate::mock::Test);
}
