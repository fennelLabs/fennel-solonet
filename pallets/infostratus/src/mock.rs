use crate as pallet_infostratus;
use frame_support::{derive_impl, parameter_types};
use sp_core::ConstU32;
use sp_runtime::BuildStorage;

pub type Balance = u128;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
#[frame_support::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;
    #[runtime::pallet_index(1)]
    pub type Infostratus = pallet_infostratus::Pallet<Test>;
    #[runtime::pallet_index(2)]
    pub type Balances = pallet_balances::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
	pub const MockLockIdentifier: [u8; 8] = *b"infolock";
}

impl pallet_balances::Config for Test {
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type MaxLocks = ();
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type RuntimeHoldReason = ();
	type FreezeIdentifier = [u8; 8];
	type MaxFreezes = ();
	type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

impl pallet_infostratus::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Currency = Balances;
	type MaxSize = ConstU32<1024>;
	type LockId = MockLockIdentifier;
	type LockPrice = ConstU32<10>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap()
		.into()
}
