# Genesis Configuration for Validator Setup

This document explains the genesis configuration approach for setting up validators in the Fennel Solonet chain.

## Overview

The Fennel Solonet chain uses a genesis configuration approach for setting up validators, which provides the following advantages:

1. **Deterministic Startup**: Validators are configured from block 0
2. **Easier Automation**: No manual steps required for initial validators
3. **Better Testability**: Simplifies Zombienet testing and container orchestration
4. **More Resilient**: Less prone to configuration errors during deployment

## How It Works

The genesis configuration includes both the `session` and `validator-manager` pallets with initial validator configurations:

1. **Session Keys**: The session pallet is configured with initial validators' session keys, mapping account IDs to their respective AuRa and GRANDPA keys.
2. **Validator Manager**: The validator-manager pallet is configured with initial validator account IDs.

## Implementation Details

The genesis configuration is defined in the `testnet_genesis` function in `node/src/chain_spec.rs`. This function creates a JSON configuration that includes:

```rust
"session": {
    "keys": [/* Array of (account_id, account_id, session_keys) tuples */]
},
"validatorManager": {
    "validators": [/* Array of validator account IDs */]
}
```

## Dynamic Validator Management

Despite having validators configured at genesis, the chain still supports dynamic validator management:

1. **Adding Validators**: Use the validator-manager pallet's `register_validators` extrinsic
2. **Removing Validators**: Use the validator-manager pallet's `remove_validator` extrinsic

These changes take effect after two session rotations.

## Testing and Deployment Benefits

1. **Zombienet Testing**: Nodes launched by Zombienet will immediately be validators
2. **Containerization**: Container images have validators configured from the start
3. **Kubernetes**: More predictable StatefulSet behavior without bootstrap phases

## Recommended Usage

When setting up a new Fennel Solonet chain:

1. Configure the initial validator set in the genesis configuration
2. Use this genesis configuration to start all nodes
3. For adding validators later, use the `register_validators` extrinsic
4. For removing validators, use the `remove_validator` extrinsic

This approach balances initial determinism with runtime flexibility. 