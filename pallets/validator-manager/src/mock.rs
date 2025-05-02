#![cfg(test)]

use crate as pallet_validator_manager;
use codec::{Decode, Encode};
use frame_support::{
    parameter_types,
    traits::{ConstU16, ConstU64, Hooks},
};
use frame_system as system;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::KeyTypeId, H256};
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage, Perbill,
};

// Define key types for session keys
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"test");

// Define the key type ids here
mod keys {
    use super::KEY_TYPE;
    pub const DUMMY: super::KeyTypeId = KEY_TYPE;
}

// Mock session keys
#[derive(
    Debug,
    Clone,
    Encode,
    Decode,
    PartialEq,
    Eq,
    scale_info::TypeInfo,
    Serialize,
    Deserialize,
    Default,
)]
pub struct MockSessionKeys {
    pub dummy: [u8; 32],
}

impl sp_runtime::traits::OpaqueKeys for MockSessionKeys {
    type KeyTypeIdProviders = ();

    fn key_ids() -> &'static [KeyTypeId] {
        &[keys::DUMMY]
    }

    fn get_raw(&self, i: KeyTypeId) -> &[u8] {
        if i == keys::DUMMY {
            &self.dummy[..]
        } else {
            &[]
        }
    }
}

// Mock session handler for validator
pub struct MockSessionHandler;
impl pallet_session::SessionHandler<u64> for MockSessionHandler {
    const KEY_TYPE_IDS: &'static [KeyTypeId] = &[keys::DUMMY];

    fn on_genesis_session<Ks: sp_runtime::traits::OpaqueKeys>(_validators: &[(u64, Ks)]) {}

    fn on_new_session<Ks: sp_runtime::traits::OpaqueKeys>(
        _changed: bool,
        _validators: &[(u64, Ks)],
        _queued_validators: &[(u64, Ks)],
    ) {
    }

    fn on_disabled(_validator_index: u32) {}
}

type Block = frame_system::mocking::MockBlock<Test>;
type AccountId = u64;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>},
        ValidatorManager: pallet_validator_manager,
    }
);

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type RuntimeTask = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

parameter_types! {
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(33);
    pub const MinAuthorities: u32 = 1;
    pub const Period: u64 = 1;
    pub const Offset: u64 = 0;
}

impl pallet_session::Config for Test {
    type ValidatorId = AccountId;
    type ValidatorIdOf = pallet_validator_manager::ValidatorOf<Test>;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = ValidatorManager;
    type SessionHandler = MockSessionHandler;
    type Keys = MockSessionKeys;
    type WeightInfo = ();
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_validator_manager::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type PrivilegedOrigin = frame_system::EnsureRoot<AccountId>;
    type MinAuthorities = MinAuthorities;
    type WeightInfo = ();
    type ValidatorOf = pallet_validator_manager::ValidatorOf<Test>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    let validators = vec![1, 2, 3];
    // Create unique session keys for each validator
    let session_keys: Vec<_> = validators
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let mut sk = MockSessionKeys::default();
            // Ensure unique keys by setting some bytes based on index
            sk.dummy[0] = i as u8;
            (v, sk)
        })
        .collect();

    pallet_session::GenesisConfig::<Test> {
        keys: session_keys
            .into_iter()
            .map(|(account_id, session_keys)| (account_id, account_id, session_keys))
            .collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        System::set_block_number(1);
        // Initialize validators using the Hooks trait
        <Session as Hooks<u64>>::on_initialize(1);
    });
    ext
}
