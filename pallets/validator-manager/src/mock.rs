//! Test utilities for validator-manager pallet

use crate as pallet_validator_manager;
use frame_support::parameter_types;
use frame_support::traits::{OneSessionHandler, ConstU64};
use sp_runtime::BuildStorage;
use sp_runtime::testing::UintAuthorityId;
use sp_runtime::BoundToRuntimeAppPublic;
use pallet_session::PeriodicSessions;
use pallet_balances as balances;
use frame_support::pallet_prelude::{ConstU32, MaxEncodedLen};
use sp_runtime::Perbill;
use codec::{Encode, Decode, DecodeWithMemTracking};
use scale_info::TypeInfo;
use sp_std::fmt;

// Use u64 for AccountId for simplicity in tests
pub type AccountId = ValidatorId;

#[cfg_attr(any(feature = "std", test), derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Encode, Decode, TypeInfo, MaxEncodedLen, PartialOrd, Ord, DecodeWithMemTracking)]
pub struct ValidatorId(pub u64);

impl From<u64> for ValidatorId {
    fn from(x: u64) -> Self {
        ValidatorId(x)
    }
}

impl fmt::Display for ValidatorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type Block = frame_system::mocking::MockBlock<Test>;

// Dummy session handler for tests
pub struct DummySessionHandler;
impl BoundToRuntimeAppPublic for DummySessionHandler {
    type Public = UintAuthorityId;
}
impl OneSessionHandler<ValidatorId> for DummySessionHandler {
    type Key = UintAuthorityId;
    fn on_genesis_session<'a, I: Iterator<Item = (&'a ValidatorId, Self::Key)>>(_validators: I) {}
    fn on_new_session<'a, I: Iterator<Item = (&'a ValidatorId, Self::Key)>>(_changed: bool, _validators: I, _queued_validators: I) {}
    fn on_disabled(_validator_index: u32) {}
}

frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system::{Pallet, Call, Storage, Event<T>},
        Session: pallet_session::{Pallet, Call, Storage, Event<T>},
        ValidatorManager: pallet_validator_manager::{Pallet, Call, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
    }
);

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MinAuthorities: u32 = 2;
    pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights::with_sensible_defaults(
        frame_support::weights::Weight::from_parts(2u64 * frame_support::weights::constants::WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
        NORMAL_DISPATCH_RATIO,
    );
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const ExistentialDeposit: u64 = 1;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = BlockWeights;
    type BlockLength = BlockLength;
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type Hash = sp_core::H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type RuntimeTask = ();
    type Nonce = u64;
    type Block = frame_system::mocking::MockBlock<Test>;
    type ExtensionsWeightInfo = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ConstU32<50>;
    type ReserveIdentifier = [u8; 8];
    type FreezeIdentifier = [u8; 8];
    type MaxFreezes = ConstU32<50>;
    type RuntimeHoldReason = (); // Not used in tests
    type RuntimeFreezeReason = (); // Not used in tests
    type DoneSlashHandler = (); // Not used in tests
}

impl pallet_session::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = ValidatorId;
    type ValidatorIdOf = crate::ValidatorOf<Test>;
    type ShouldEndSession = PeriodicSessions<ConstU64<1>, ConstU64<0>>;
    type NextSessionRotation = PeriodicSessions<ConstU64<1>, ConstU64<0>>;
    type SessionManager = ValidatorManager;
    type SessionHandler = (DummySessionHandler,);
    type Keys = UintAuthorityId;
    type WeightInfo = ();
    type DisablingStrategy = ();
}

impl pallet_validator_manager::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type PrivilegedOrigin = frame_system::EnsureRoot<AccountId>;
    type MinAuthorities = MinAuthorities;
    type ValidatorOf = crate::ValidatorOf<Test>;
    type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
    pallet_validator_manager::GenesisConfig::<Test> {
        initial_validators: vec![1, 2, 3].into_iter().map(ValidatorId).collect::<Vec<ValidatorId>>(),
    }
    .assimilate_storage(&mut t)
    .unwrap();
    pallet_session::GenesisConfig::<Test> {
        keys: vec![
            (ValidatorId(1).into(), ValidatorId(1), UintAuthorityId(1)),
            (ValidatorId(2).into(), ValidatorId(2), UintAuthorityId(2)),
            (ValidatorId(3).into(), ValidatorId(3), UintAuthorityId(3)),
        ],
        non_authority_keys: vec![],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    balances::GenesisConfig::<Test> {
        balances: vec![(ValidatorId(1), 1000), (ValidatorId(2), 1000), (ValidatorId(3), 1000), (ValidatorId(4), 1000)],
        ..Default::default()
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}