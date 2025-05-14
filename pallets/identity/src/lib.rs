#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

pub use pallet::*;
use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::{DispatchResultWithPostInfo, DispatchResult}, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::One;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
        type MaxSize: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::type_value]
    pub fn DefaultCurrent<T: Config>() -> u32 { 0 }

    #[pallet::storage]
    #[pallet::getter(fn identity_number)]
    pub type IdentityNumber<T: Config> =
        StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;

    #[pallet::storage]
    #[pallet::getter(fn get_signal_count)]
    pub type SignalCount<T: Config> =
        StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;

    #[pallet::storage]
    #[pallet::getter(fn identity_list)]
    pub type IdentityList<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn identity_trait_list)]
    pub type IdentityTraitList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxSize>,
        BoundedVec<u8, T::MaxSize>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        IdentityCreated { identity_id: u32, owner: T::AccountId },
        IdentityRevoked { identity_id: u32, owner: T::AccountId },
        IdentityUpdated { identity_id: u32, owner: T::AccountId },
    }

    #[pallet::error]
    #[derive(PartialEq, Eq)]
    pub enum Error<T> {
        StorageOverflow,
        IdentityNotOwned,
    }

    impl<T: Config> Pallet<T> {
        fn is_identity_owned_by_sender(account_id: &T::AccountId, identity_id: &u32) -> bool {
            match <IdentityList<T>>::try_get(identity_id) {
                Result::Ok(owner) => owner == *account_id,
                Result::Err(_) => false,
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::create_identity())]
        #[pallet::call_index(0)]
        pub fn create_identity(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_id: u32 = <IdentityNumber<T>>::get();
            <IdentityNumber<T>>::try_mutate(|current_id| -> DispatchResult {
                *current_id = current_id.checked_add(One::one()).ok_or(Error::<T>::StorageOverflow)?;
                Ok(())
            })?;
            let new_id: u32 = <IdentityNumber<T>>::get();
            ensure!(!<IdentityList<T>>::contains_key(&current_id), Error::<T>::StorageOverflow);
            <IdentityList<T>>::try_mutate(&current_id, |owner| -> DispatchResult {
                *owner = Some(who.clone());
                Ok(())
            })?;
            <IdentityNumber<T>>::put(new_id);
            Self::deposit_event(Event::IdentityCreated { identity_id: current_id, owner: who.clone() });
            Ok(().into())
        }
        #[pallet::weight(T::WeightInfo::revoke_identity())]
        #[pallet::call_index(1)]
        pub fn revoke_identity(origin: OriginFor<T>, identity_id: u32) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_identity_owned_by_sender(&who, &identity_id), Error::<T>::IdentityNotOwned);
            <IdentityList<T>>::try_mutate(&identity_id, |owner| -> DispatchResult {
                *owner = None;
                Ok(())
            })?;
            Self::deposit_event(Event::IdentityRevoked { identity_id, owner: who.clone() });
            Ok(().into())
        }
        #[pallet::weight(T::WeightInfo::add_or_update_identity_trait(key.len() as u32))]
        #[pallet::call_index(2)]
        pub fn add_or_update_identity_trait(
            origin: OriginFor<T>,
            identity_id: u32,
            key: BoundedVec<u8, T::MaxSize>,
            value: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_identity_owned_by_sender(&who, &identity_id), Error::<T>::IdentityNotOwned);
            <IdentityTraitList<T>>::try_mutate(identity_id, key.clone(), |v| -> DispatchResult {
                *v = value;
                Ok(())
            })?;
            Self::deposit_event(Event::IdentityUpdated { identity_id, owner: who.clone() });
            Ok(().into())
        }
        #[pallet::weight(T::WeightInfo::remove_identity_trait(key.len() as u32))]
        #[pallet::call_index(3)]
        pub fn remove_identity_trait(
            origin: OriginFor<T>,
            identity_id: u32,
            key: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_identity_owned_by_sender(&who, &identity_id), Error::<T>::IdentityNotOwned);
            <IdentityTraitList<T>>::remove(identity_id, key.clone());
            Self::deposit_event(Event::IdentityUpdated { identity_id, owner: who.clone() });
            Ok(().into())
        }
    }
}
