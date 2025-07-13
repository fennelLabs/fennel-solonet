// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{AccountId, BalancesConfig, RuntimeGenesisConfig, SudoConfig};
use alloc::{vec, vec::Vec};
use frame_support::build_struct_json_patch;
use serde_json::Value;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_genesis_builder::{self, PresetId};
use sp_keyring::Sr25519Keyring;
use crate::SessionKeys;
use sp_core::{crypto::Ss58Codec, ByteArray};
use sp_core::crypto::AccountId32;

/// Identifier for the staging preset.
pub const STAGING_RUNTIME_PRESET: &str = "staging";

/// Identifier for the production preset.
pub const PRODUCTION_RUNTIME_PRESET: &str = "production";

// Helper function to parse hex-encoded public key from environment variable
fn parse_aura_public_key(hex_str: &str) -> AuraId {
	let bytes = hex::decode(hex_str.trim_start_matches("0x"))
		.expect("Invalid hex in Aura public key");
	AuraId::from_slice(&bytes)
		.expect("Invalid Aura public key format")
}

// Helper function to parse hex-encoded GRANDPA public key from environment variable  
fn parse_grandpa_public_key(hex_str: &str) -> GrandpaId {
	let bytes = hex::decode(hex_str.trim_start_matches("0x"))
		.expect("Invalid hex in GRANDPA public key");
	GrandpaId::from_slice(&bytes)
		.expect("Invalid GRANDPA public key format")
}

// Helper function to parse SS58 account ID from environment variable
fn parse_account_id(ss58_str: &str) -> AccountId {
	AccountId32::from_ss58check(ss58_str)
		.expect("Invalid SS58 account ID")
}

// Returns the genesis config for production with proper validator handling
fn production_genesis(
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	validator_stash_accounts: Vec<AccountId>,
	root: AccountId,
	endowed_accounts: Vec<AccountId>,
) -> Value {
	// Production token distribution - all tokens to sudo account initially
	let total_supply = 1_000_000_000_000_000_000u128; // 1 billion FNL tokens (18 decimals)
	
	let balances: Vec<(AccountId, u128)> = endowed_accounts
		.iter()
		.map(|account| (account.clone(), total_supply))
		.collect();
	
	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances,
		},
		sudo: SudoConfig { key: Some(root.clone()) },
		validator_manager: pallet_validator_manager::GenesisConfig {
			initial_validators: validator_stash_accounts.clone(),
		},
		session: pallet_session::GenesisConfig {
			keys: initial_authorities.iter().zip(validator_stash_accounts.iter()).map(|(session_keys, stash_account)| {
				(
					stash_account.clone(),
					stash_account.clone(),
					SessionKeys {
						aura: session_keys.0.clone(),
						grandpa: session_keys.1.clone(),
					}
				)
			}).collect::<Vec<_>>(),
		},
	})
}

// Returns the genesis config presets populated with given parameters.
fn testnet_genesis(
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root: AccountId,
	endowed_accounts: Vec<AccountId>,
) -> Value {
	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1u128 << 60))  // Everyone gets the same amount: 2^60 tokens (~1.15 billion)
				.collect::<Vec<_>>(),
		},
		sudo: SudoConfig { key: Some(root.clone()) },
		validator_manager: pallet_validator_manager::GenesisConfig {
			initial_validators: initial_authorities.iter().map(|x| {
				if x.0 == sp_keyring::Sr25519Keyring::Alice.public().into() {
					Sr25519Keyring::Alice.to_account_id()
				} else if x.0 == sp_keyring::Sr25519Keyring::Bob.public().into() {
					Sr25519Keyring::Bob.to_account_id()
				} else {
					panic!("Unknown authority in initial_authorities");
				}
			}).collect::<Vec<_>>(),
		},
		session: pallet_session::GenesisConfig {
			keys: initial_authorities.iter().map(|x| {
				let account_id = if x.0 == sp_keyring::Sr25519Keyring::Alice.public().into() {
					Sr25519Keyring::Alice.to_account_id()
				} else if x.0 == sp_keyring::Sr25519Keyring::Bob.public().into() {
					Sr25519Keyring::Bob.to_account_id()
				} else {
					panic!("Unknown authority in initial_authorities");
				};
				(
					account_id.clone(),
					account_id.clone(),
					SessionKeys {
						aura: x.0.clone(),
						grandpa: x.1.clone(),
					}
				)
			}).collect::<Vec<_>>(),
		},
	})
}

/// Return the development genesis config.
pub fn development_config_genesis() -> Value {
	testnet_genesis(
		vec![(
			sp_keyring::Sr25519Keyring::Alice.public().into(),
			sp_keyring::Ed25519Keyring::Alice.public().into(),
		)],
		sp_keyring::Sr25519Keyring::Alice.to_account_id(),
		vec![
			Sr25519Keyring::Alice.to_account_id(),
			Sr25519Keyring::Bob.to_account_id(),
			Sr25519Keyring::AliceStash.to_account_id(),
			Sr25519Keyring::BobStash.to_account_id(),
		],
	)
}

/// Return the local genesis config preset.
pub fn local_config_genesis() -> Value {
	testnet_genesis(
		vec![
			(
				sp_keyring::Sr25519Keyring::Alice.public().into(),
				sp_keyring::Ed25519Keyring::Alice.public().into(),
			),
			(
				sp_keyring::Sr25519Keyring::Bob.public().into(),
				sp_keyring::Ed25519Keyring::Bob.public().into(),
			),
		],
		Sr25519Keyring::Alice.to_account_id(),
		Sr25519Keyring::iter()
			.filter(|v| v != &Sr25519Keyring::One && v != &Sr25519Keyring::Two)
			.map(|v| v.to_account_id())
			.collect::<Vec<_>>(),
	)
}

/// Return the staging genesis config preset.
pub fn staging_config_genesis() -> Value {
	testnet_genesis(
		// 1) Authorities: Alice & Bob Aura/Grandpa IDs
		vec![
			(
				Sr25519Keyring::Alice.public().into(),
				sp_keyring::Ed25519Keyring::Alice.public().into(),
			),
			(
				Sr25519Keyring::Bob.public().into(),
				sp_keyring::Ed25519Keyring::Bob.public().into(),
			),
		],
		// 2) Sudo/root key: Alice
		Sr25519Keyring::Alice.to_account_id(),
		// 3) Endowed accounts (Alice, Bob, and their stashes)
		vec![
			Sr25519Keyring::Alice.to_account_id(),
			Sr25519Keyring::Bob.to_account_id(),
			Sr25519Keyring::AliceStash.to_account_id(),
			Sr25519Keyring::BobStash.to_account_id(),
		],
	)
}

/// Return the production genesis config preset.
/// Uses compile-time environment variables for all public keys and account IDs.
/// Private keys are managed separately via Vault at runtime.
/// 
/// This function is only available when all required environment variables are set at compile time.
pub fn production_config_genesis() -> Value {
	// Parse production validator keys from environment variables
	let val1_aura = parse_aura_public_key(env!("VAL1_AURA_PUB"));
	let val1_grandpa = parse_grandpa_public_key(env!("VAL1_GRANDPA_PUB"));
	let val2_aura = parse_aura_public_key(env!("VAL2_AURA_PUB"));
	let val2_grandpa = parse_grandpa_public_key(env!("VAL2_GRANDPA_PUB"));
	
	// Parse production account IDs from environment variables
	let sudo_account = parse_account_id(env!("SUDO_SS58"));
	let val1_stash = parse_account_id(env!("VAL1_STASH_SS58"));
	let val2_stash = parse_account_id(env!("VAL2_STASH_SS58"));
	
	production_genesis(
		// 1) Authorities: Production validator session keys from environment
		vec![
			(val1_aura, val1_grandpa),
			(val2_aura, val2_grandpa),
		],
		// 2) Validators: Production stash accounts from environment
		vec![val1_stash.clone(), val2_stash.clone()],
		// 3) Sudo/root key: Production admin account from environment
		sudo_account.clone(),
		// 4) Endowed accounts: Give all tokens to sudo account for centralized initial distribution
		vec![sudo_account],
	)
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
	let patch = match id.as_ref() {
		sp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
		sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => local_config_genesis(),
		STAGING_RUNTIME_PRESET => staging_config_genesis(),
		PRODUCTION_RUNTIME_PRESET => {
			// Only allow production preset if all environment variables are available
			if option_env!("SUDO_SS58").is_some() &&
				option_env!("VAL1_AURA_PUB").is_some() &&
				option_env!("VAL1_GRANDPA_PUB").is_some() &&
				option_env!("VAL1_STASH_SS58").is_some() &&
				option_env!("VAL2_AURA_PUB").is_some() &&
				option_env!("VAL2_GRANDPA_PUB").is_some() &&
				option_env!("VAL2_STASH_SS58").is_some() {
				production_config_genesis()
			} else {
				return None;
			}
		},
		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}

/// List of supported presets.
pub fn preset_names() -> Vec<PresetId> {
	let mut presets = vec![
		PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
		PresetId::from(STAGING_RUNTIME_PRESET),
	];
	
	// Only include production preset if all required environment variables are available
	// Check if production environment variables are set at compile time
	if option_env!("SUDO_SS58").is_some() &&
		option_env!("VAL1_AURA_PUB").is_some() &&
		option_env!("VAL1_GRANDPA_PUB").is_some() &&
		option_env!("VAL1_STASH_SS58").is_some() &&
		option_env!("VAL2_AURA_PUB").is_some() &&
		option_env!("VAL2_GRANDPA_PUB").is_some() &&
		option_env!("VAL2_STASH_SS58").is_some() {
		presets.push(PresetId::from(PRODUCTION_RUNTIME_PRESET));
	}
	
	presets
}
