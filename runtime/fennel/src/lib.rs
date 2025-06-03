#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod apis;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarks;
pub mod configs;

extern crate alloc;
use alloc::vec::Vec;
use sp_runtime::{
	generic, impl_opaque_keys,
	traits::{BlakeTwo256, IdentifyAccount, Verify, Convert},
	MultiAddress, MultiSignature,
};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
use frame_support::parameter_types;
use pallet_session::PeriodicSessions;
use pallet_validator_manager::{self, ValidatorOf};
use frame_support::pallet_prelude::ConstU32;

pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
pub use pallet_session::historical as session_historical;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

pub mod genesis_config_presets;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;
	use sp_runtime::{
		generic,
		traits::{BlakeTwo256, Hash as HashT},
	};

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
	/// Opaque block hash type.
	pub type Hash = <BlakeTwo256 as HashT>::Output;
}

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub aura: Aura,
			pub grandpa: Grandpa,
		}
	}

// To learn more about runtime versioning, see:
// https://docs.substrate.io/main-docs/build/upgrade#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: alloc::borrow::Cow::Borrowed("solochain-runtime"),
	impl_name: alloc::borrow::Cow::Borrowed("solochain-runtime"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 100,
	impl_version: 1,
	apis: apis::RUNTIME_API_VERSIONS,
	transaction_version: 1,
	system_version: 1,
};

mod block_times {
	/// This determines the average expected block time that we are targeting. Blocks will be
	/// produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by
	/// `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn
	/// slot_duration()`.
///
/// Change this to adjust the block time.
	pub const MILLI_SECS_PER_BLOCK: u64 = 6000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
	// Attempting to do so will brick block production.
	pub const SLOT_DURATION: u64 = MILLI_SECS_PER_BLOCK;
}
pub use block_times::*;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLI_SECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

pub const BLOCK_HASH_COUNT: BlockNumber = 2400;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLI_UNIT: Balance = 1_000_000_000;
pub const MICRO_UNIT: Balance = 1_000_000;

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLI_UNIT;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// The `TransactionExtension` to the basic transaction logic.
pub type TxExtension = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
	frame_metadata_hash_extension::CheckMetadataHash<Runtime>,
	frame_system::WeightReclaim<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, TxExtension>;

/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, TxExtension>;

/// All migrations of the runtime, aside from the ones declared in the pallets.
///
/// This can be a tuple of types, each implementing `OnRuntimeUpgrade`.
#[allow(unused_parens)]
type Migrations = ();

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	Migrations,
>;

/// Convert `AccountId` → session index (here we just use the AccountId itself).
pub struct ConvertAccountIdToSessionIndex;
impl Convert<AccountId, Option<AccountId>> for ConvertAccountIdToSessionIndex {
    fn convert(x: AccountId) -> Option<AccountId> { Some(x) }
}

parameter_types! {
    pub const Period: u32 = 2; // 2 blocks = ~12 seconds per session - extremely short for testing
    pub const Offset: u32 = 0;
    pub const MinAuthorities: u32 = 2;
}

impl pallet_session::Config for Runtime {
    type RuntimeEvent      = RuntimeEvent;
    type ValidatorId       = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf     = ConvertAccountIdToSessionIndex;
    type ShouldEndSession  = PeriodicSessions<Period, Offset>;
    
    // Use ValidatorManager as SessionManager
    type SessionManager    = ValidatorManager;
    // who gets new_session() callbacks - both Aura and Grandpa need to be notified
    type SessionHandler    = (Aura, Grandpa);
    
    type Keys              = SessionKeys;
    type NextSessionRotation = PeriodicSessions<Period, Offset>;
    type WeightInfo        = pallet_session::weights::SubstrateWeight<Runtime>;
    // Use the unit type () as DisablingStrategy, which is a valid implementation
    // that does nothing (no disabling)
    type DisablingStrategy = ();
}

// Configure the validator manager pallet
impl pallet_validator_manager::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type PrivilegedOrigin = frame_system::EnsureRoot<AccountId>;
    type MinAuthorities = MinAuthorities;
    type WeightInfo = pallet_validator_manager::weights::SubstrateWeight<Runtime>;
    type ValidatorOf = ValidatorOf<Runtime>;
}

parameter_types! {
    pub const CertificateLockId: [u8; 8] = *b"certlock";
    pub const CertificateLockPrice: u32 = 4_294_967_295; // max u32 value
}

impl pallet_certificate::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_certificate::weights::SubstrateWeight<Runtime>;
    type Currency = pallet_balances::Pallet<Runtime>;
    type LockId = CertificateLockId;
    type LockPrice = CertificateLockPrice;
}

// Implement the Config trait for the identity pallet in the runtime
impl pallet_identity::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_identity::weights::SubstrateWeight<Runtime>;
    type MaxSize = ConstU32<1024>; // Updated to match original codebase
}

// Implement the Config trait for the keystore pallet in the runtime
impl pallet_keystore::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_keystore::weights::SubstrateWeight<Runtime>;
    type MaxSize = ConstU32<1024>; // Updated to match original codebase
}

parameter_types! {
    pub const InfostratusLockId: [u8; 8] = *b"infosloc";
    pub const InfostratusLockPrice: u32 = 1_000_000_000; // set as needed
    pub const InfostratusMaxSize: u32 = 1024; // Updated to match original codebase
}

impl pallet_infostratus::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_infostratus::weights::SubstrateWeight<Runtime>;
    type Currency = pallet_balances::Pallet<Runtime>;
    type LockId = InfostratusLockId;
    type LockPrice = InfostratusLockPrice;
    type MaxSize = InfostratusMaxSize;
}

parameter_types! {
    pub const SignalLockId: [u8; 8] = *b"signallk";
    pub const SignalLockPrice: u32 = 1_000_000_000; // set as needed
    pub const SignalMaxSize: u32 = 1024; // Updated to match original codebase
}

impl pallet_signal::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_signal::weights::SubstrateWeight<Runtime>;
    type Currency = pallet_balances::Pallet<Runtime>;
    type LockId = SignalLockId;
    type LockPrice = SignalLockPrice;
    type MaxSize = SignalMaxSize;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
#[frame_support::runtime]
pub mod runtime {
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
		RuntimeTask,
		RuntimeViewFunction
	)]
	pub struct Runtime;

	#[runtime::pallet_index(0)]
	pub type System = frame_system;

	#[runtime::pallet_index(1)]
	pub type Timestamp = pallet_timestamp;

	#[runtime::pallet_index(2)]
	pub type Aura = pallet_aura;

	#[runtime::pallet_index(3)]
	pub type Grandpa = pallet_grandpa;

	#[runtime::pallet_index(4)]
	pub type Balances = pallet_balances;

	#[runtime::pallet_index(5)]
	pub type TransactionPayment = pallet_transaction_payment;

	#[runtime::pallet_index(6)]
	pub type Sudo = pallet_sudo;

	#[runtime::pallet_index(7)]
	pub type ValidatorManager = pallet_validator_manager;

	#[runtime::pallet_index(8)]
	pub type Session = pallet_session;

	#[runtime::pallet_index(9)]
	pub type Certificate = pallet_certificate;

	#[runtime::pallet_index(10)]
	pub type Identity = pallet_identity;

	#[runtime::pallet_index(11)]
	pub type Keystore = pallet_keystore;

	#[runtime::pallet_index(12)]
	pub type Infostratus = pallet_infostratus;

	#[runtime::pallet_index(13)]
	pub type Signal = pallet_signal;

	#[runtime::pallet_index(14)]
	pub type Trust = pallet_trust;
}

// No need for explicit re-export as the module is now public
