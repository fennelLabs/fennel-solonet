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

/// Identifier for the staging preset.
pub const STAGING_RUNTIME_PRESET: &str = "staging";

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
				.map(|k| (k, 1u128 << 60))
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

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
	let patch = match id.as_ref() {
		sp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
		sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => local_config_genesis(),
		STAGING_RUNTIME_PRESET => staging_config_genesis(),
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
	vec![
		PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
		PresetId::from(STAGING_RUNTIME_PRESET),
	]
}
