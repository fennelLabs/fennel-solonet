#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

const ASSIGNMENT_EXISTS: bool = true;
const ASSIGNMENT_DOES_NOT_EXIST: bool = false;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::{DispatchResultWithPostInfo, DispatchResult},
        pallet_prelude::*,
        traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons},
    };
    use frame_system::pallet_prelude::*;

    use crate::{weights::WeightInfo, ASSIGNMENT_DOES_NOT_EXIST, ASSIGNMENT_EXISTS};

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
        type Currency: LockableCurrency<
            Self::AccountId,
            Moment = frame_system::pallet_prelude::BlockNumberFor<Self>,
        >;
        type MaxSize: Get<u32>;
        type LockId: Get<LockIdentifier>;
        type LockPrice: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn assignments_list)]
    pub type AssignmentsList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxSize>,
        bool,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn submissions_list)]
    pub type SubmissionsList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxSize>,
        bool,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SubmissionSent { who: T::AccountId, resource_location: BoundedVec<u8, T::MaxSize> },
        SubmissionAssigned { resource_location: BoundedVec<u8, T::MaxSize>, who: T::AccountId },
        InfostratusLock { account: T::AccountId, amount: BalanceOf<T> },
        InfostratusUnlock { account: T::AccountId, amount: BalanceOf<T> },
    }

    #[pallet::error]
    pub enum Error<T> {
        SubmissionDoesNotExist,
        SubmissionExists,
        SubmissionAlreadyAssigned,
        InsufficientBalance,
        CannotAssignOwnSubmission,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::create_submission_entry())]
        #[pallet::call_index(0)]
        pub fn create_submission_entry(
            origin: OriginFor<T>,
            resource_location: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            if T::Currency::total_balance(&who) < T::Currency::minimum_balance() {
                return Err(Error::<T>::InsufficientBalance.into());
            }
            ensure!(
                !SubmissionsList::<T>::contains_key(&who, &resource_location),
                Error::<T>::SubmissionExists
            );
            T::Currency::set_lock(T::LockId::get(), &who, 10u32.into(), WithdrawReasons::all());
            Self::deposit_event(Event::InfostratusLock { account: who.clone(), amount: T::Currency::free_balance(&who) });
            <SubmissionsList<T>>::try_mutate(
                &who,
                resource_location.clone(),
                |value| -> DispatchResult {
                    *value = ASSIGNMENT_DOES_NOT_EXIST;
                    Ok(())
                },
            )?;
            Self::deposit_event(Event::SubmissionSent { who, resource_location });
            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::request_submission_assignment())]
        #[pallet::call_index(1)]
        pub fn request_submission_assignment(
            origin: OriginFor<T>,
            poster: T::AccountId,
            resource_location: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            if T::Currency::total_balance(&who) < T::Currency::minimum_balance() {
                return Err(Error::<T>::InsufficientBalance.into());
            }
            ensure!(&who != &poster, Error::<T>::CannotAssignOwnSubmission);
            ensure!(
                SubmissionsList::<T>::contains_key(&poster, &resource_location),
                Error::<T>::SubmissionDoesNotExist
            );
            ensure!(
                !SubmissionsList::<T>::get(&poster, &resource_location),
                Error::<T>::SubmissionAlreadyAssigned
            );
            T::Currency::set_lock(T::LockId::get(), &who, 10u32.into(), WithdrawReasons::all());
            Self::deposit_event(Event::InfostratusUnlock { account: who.clone(), amount: T::Currency::free_balance(&who) });
            <AssignmentsList<T>>::try_mutate(
                &who,
                resource_location.clone(),
                |value| -> DispatchResult {
                    *value = ASSIGNMENT_EXISTS;
                    Ok(())
                },
            )?;
            <SubmissionsList<T>>::try_mutate(
                &poster,
                resource_location.clone(),
                |value| -> DispatchResult {
                    *value = ASSIGNMENT_EXISTS;
                    Ok(())
                },
            )?;
            Self::deposit_event(Event::SubmissionAssigned { resource_location, who });
            Ok(().into())
        }
    }
}
