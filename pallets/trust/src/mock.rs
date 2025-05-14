use crate as pallet_trust;
use frame_support::derive_impl;
use sp_core::ConstU32;
use sp_runtime::BuildStorage;

pub type AccountId = u64;
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
    pub type Trust = pallet_trust::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
    type AccountId = AccountId;
}

impl pallet_trust::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MaxTrustParameterSize = ConstU32<1024>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap()
		.into()
}
