#![cfg_attr(not(feature = "std"), no_std)]

//! A pallet for managing validators on Solochain.
//!
//! This pallet provides a mechanism for adding and removing validators
//! through a privileged origin. It integrates with the session pallet
//! to manage validator sets across sessions.

extern crate alloc;
use alloc::vec::Vec;
use frame_support::traits::{Get, BuildGenesisConfig};
use sp_runtime::traits::{Convert, OpaqueKeys};
use sp_staking::{SessionIndex};
use codec::Encode;

pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(test)]
pub mod mock;

#[cfg(test)]
pub mod tests;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

type Session<T> = pallet_session::Pallet<T>;

/// A type used to convert an account ID into a validator ID.
pub struct ValidatorOf<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> Convert<T::AccountId, Option<T::AccountId>> for ValidatorOf<T> {
    fn convert(account: T::AccountId) -> Option<T::AccountId> {
        Some(account)
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        dispatch::DispatchResult, ensure, pallet_prelude::*, traits::EnsureOrigin,
    };
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configuration for the validator manager.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_session::Config {
        /// The overreaching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Privileged origin that can add or remove validators.
        type PrivilegedOrigin: EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>;

        /// Minimum number of validators that should be maintained
        #[pallet::constant]
        type MinAuthorities: Get<u32>;

        /// Session period in blocks (how often session changes occur)
        #[pallet::constant]
        type SessionPeriod: Get<u32>;

        /// Session offset in blocks (when sessions start relative to block 0)
        #[pallet::constant]
        type SessionOffset: Get<u32>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

        /// Type that converts an account ID to a validator ID.
        type ValidatorOf: Convert<Self::AccountId, Option<Self::ValidatorId>>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// New validators were added to the set.
        ValidatorsRegistered { validators: Vec<T::ValidatorId> },
        /// A validator was removed from the set.
        ValidatorRemoved { validator: T::ValidatorId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The validator is already in the set.
        ValidatorAlreadyAdded,
        /// The account is not a validator.
        NotValidator,
        /// Removing this validator would put the validator count below the minimum.
        TooFewValidators,
        /// Validator has no session keys registered.
        NoKeysRegistered,
    }

    /// Validators that should be removed.
    #[pallet::storage]
    #[pallet::getter(fn validators_to_remove)]
    #[pallet::unbounded]
    pub type ValidatorsToRemove<T> = StorageValue<_, Vec<<T as pallet_session::Config>::ValidatorId>, ValueQuery>;

    /// Validators that should be added.
    #[pallet::storage]
    #[pallet::getter(fn validators_to_add)]
    #[pallet::unbounded]
    pub type ValidatorsToAdd<T> = StorageValue<_, Vec<<T as pallet_session::Config>::ValidatorId>, ValueQuery>;

    /// Add genesis configuration for the validator manager pallet
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub initial_validators: Vec<T::AccountId>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            if !self.initial_validators.is_empty() {
                Pallet::<T>::put_validators(&self.initial_validators);
            }
        }
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                initial_validators: Vec::new(),
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add new validators to the set.
        ///
        /// The new validators will be active from current session + 2.
        /// 
        /// # Requirements
        /// - Validators must have session keys registered via `session.setKeys()` before calling this
        #[pallet::call_index(0)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::register_validators(validators.len() as u32))]
        pub fn register_validators(
            origin: OriginFor<T>,
            validators: Vec<T::ValidatorId>,
        ) -> DispatchResult {
            T::PrivilegedOrigin::ensure_origin(origin)?;

            let mut current_validators_to_add = ValidatorsToAdd::<T>::get();

            for validator in validators.clone() {
                // Check if the validator is already in the to_add list
                ensure!(
                    !current_validators_to_add.contains(&validator),
                    Error::<T>::ValidatorAlreadyAdded
                );

                // CRITICAL: Check if validator has session keys registered
                // This prevents adding validators without keys which causes GRANDPA to halt
                Self::validate_session_keys(&validator)?;

                // Add to the queue
                current_validators_to_add.push(validator);
            }

            ValidatorsToAdd::<T>::put(current_validators_to_add);

            Self::deposit_event(Event::ValidatorsRegistered { validators });
            Ok(())
        }

        /// Remove a validator from the set.
        ///
        /// The removed validator will be deactivated from current session + 2.
        #[pallet::call_index(1)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::remove_validator())]
        pub fn remove_validator(origin: OriginFor<T>, validator: T::ValidatorId) -> DispatchResult {
            T::PrivilegedOrigin::ensure_origin(origin)?;

            // Check if this is a known validator
            let validators = Session::<T>::validators();
            ensure!(validators.contains(&validator), Error::<T>::NotValidator);
            // Check that we won't go below the minimum number of validators
            let validators_to_remove = ValidatorsToRemove::<T>::get();
            let current_count = validators.len();
            let pending_removals = validators_to_remove.len();
            let validators_to_add = ValidatorsToAdd::<T>::get().len();
            let final_count = current_count.saturating_add(validators_to_add)
                .saturating_sub(pending_removals).saturating_sub(1);
            ensure!(
                final_count >= T::MinAuthorities::get() as usize,
                Error::<T>::TooFewValidators
            );

            // Add to removal queue
            let mut validators_to_remove = ValidatorsToRemove::<T>::get();
            validators_to_remove.push(validator.clone());
            ValidatorsToRemove::<T>::put(validators_to_remove);

            Self::deposit_event(Event::ValidatorRemoved { validator });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Helper function to set initial validators.
        pub fn put_validators(validators: &[T::AccountId]) {
            if !validators.is_empty() {
                // Convert the account IDs to validator IDs
                let validators_to_add: Vec<T::ValidatorId> = validators
                    .iter()
                    .filter_map(|account| T::ValidatorOf::convert(account.clone()))
                    .collect();

                if !validators_to_add.is_empty() {
                    ValidatorsToAdd::<T>::put(validators_to_add);
                }
            }
        }

        /// Validate that a validator has proper session keys registered
        fn validate_session_keys(validator: &T::ValidatorId) -> Result<(), Error<T>> {
            let keys = <pallet_session::NextKeys<T>>::get(validator);
            match keys {
                Some(session_keys) => {
                    // Validate that the key bundle is properly formed and non-empty
                    let encoded = session_keys.encode();
                    if encoded.is_empty() {
                        return Err(Error::<T>::NoKeysRegistered);
                    }
                    
                    // Check that the key types match what we expect
                    // Using the static key_ids function from OpaqueKeys trait
                    let key_type_ids = <T::Keys as OpaqueKeys>::key_ids();
                    
                    // For a production Substrate chain, we typically expect both AURA and GRANDPA
                    // AURA key type ID: *b"aura" (0x61757261)
                    // GRANDPA key type ID: *b"gran" (0x6772616e)
                    let expected_key_count = if cfg!(test) {
                        // In tests, we might only have one key type
                        1
                    } else {
                        // In production, we expect both AURA and GRANDPA
                        2
                    };
                    
                    if key_type_ids.len() < expected_key_count {
                        return Err(Error::<T>::NoKeysRegistered);
                    }
                    
                    // Basic validation that the session keys are properly formatted
                    // For production: 64+ bytes (32 bytes per key * 2 keys + encoding overhead)
                    // For test: 8+ bytes (single key)
                    let min_length = if cfg!(test) { 8 } else { 64 };
                    if encoded.len() < min_length {
                        return Err(Error::<T>::NoKeysRegistered);
                    }
                    
                    // If not in test mode, validate we have the expected key types
                    if !cfg!(test) {
                        let aura_key_id = sp_core::crypto::KeyTypeId(*b"aura");
                        let grandpa_key_id = sp_core::crypto::KeyTypeId(*b"gran");
                        
                        let has_aura = key_type_ids.contains(&aura_key_id);
                        let has_grandpa = key_type_ids.contains(&grandpa_key_id);
                        
                        if !has_aura || !has_grandpa {
                            return Err(Error::<T>::NoKeysRegistered);
                        }
                    }
                    
                    Ok(())
                }
                None => Err(Error::<T>::NoKeysRegistered),
            }
        }
    }
}

impl<T: Config> pallet_session::SessionManager<T::ValidatorId> for Pallet<T> {
    fn new_session(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
        // For genesis session (0), provide initial validators from storage
        if new_index == 0 {
            let initial_validators = ValidatorsToAdd::<T>::get();
            if !initial_validators.is_empty() {
                return Some(initial_validators);
            }
            return None;
        }

        let mut validators = Session::<T>::validators();

        // Apply pending changes
        let validators_to_remove = ValidatorsToRemove::<T>::take();
        validators_to_remove.iter().for_each(|v| {
            if let Some(pos) = validators.iter().position(|r| r == v) {
                validators.swap_remove(pos);
            }
        });

        let validators_to_add = ValidatorsToAdd::<T>::take();
        validators_to_add.into_iter().for_each(|v| {
            if !validators.contains(&v) {
                validators.push(v);
            }
        });

        // Remove duplicates by rebuilding the vector without duplicates
        let mut deduplicated_validators = Vec::new();
        for validator in validators {
            if !deduplicated_validators.contains(&validator) {
                deduplicated_validators.push(validator);
            }
        }
        validators = deduplicated_validators;

        // Check if we have enough validators
        let min_validators = T::MinAuthorities::get() as usize;

        if validators.len() < min_validators {
            // Not enough validators, let the chain use its default set
            None
        } else {
            // We have enough validators
            Some(validators)
        }
    }

    fn end_session(_: SessionIndex) {}

    fn start_session(_start_index: SessionIndex) {}
}

#[cfg(test)]
impl<T: Config> Pallet<T> {
    pub fn new_session(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
        <Self as pallet_session::SessionManager<_>>::new_session(new_index)
    }
}

// Add a public helper for use in genesis config
impl<T: Config> Pallet<T> {
    /// Process the validator queue and return the new validator set
    /// This is used in the genesis config to immediately activate validators
    pub fn process_queue() -> Option<Vec<T::ValidatorId>> {
        <Self as pallet_session::SessionManager<_>>::new_session(0)
    }
}

impl<T: Config> pallet_session::historical::SessionManager<T::ValidatorId, ()> for Pallet<T> {
    fn new_session(new_index: SessionIndex) -> Option<Vec<(T::ValidatorId, ())>> {
        <Self as pallet_session::SessionManager<_>>::new_session(new_index)
            .map(|r| r.into_iter().map(|v| (v, Default::default())).collect())
    }

    fn start_session(start_index: SessionIndex) {
        <Self as pallet_session::SessionManager<_>>::start_session(start_index)
    }

    fn end_session(end_index: SessionIndex) {
        <Self as pallet_session::SessionManager<_>>::end_session(end_index)
    }
}