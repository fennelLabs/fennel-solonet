# Validator Manager Pallet

A FRAME pallet that provides functionality for managing validators on Fennel Solonet.

## Overview

This pallet provides a way to add or remove validators from the validator set of a Substrate-based chain. It implements the `SessionManager` trait from the Session pallet, allowing it to update the validator set at session boundaries.

## Features

- Add validators to the validator set
- Remove validators from the validator set
- Automatic validator set updates at session boundaries
- Root-only permission for adding/removing validators

## Usage

### Extrinsics

The pallet exposes two extrinsics (dispatchable functions):

- `register_validators`: Add new validators to the set
- `deregister_validators`: Remove validators from the set

### Integration

To use this pallet in your runtime:

1. Add the validator-manager pallet to your runtime's `Cargo.toml`
2. Configure the Session pallet to use this pallet as its session manager
3. Include both pallets in your runtime

## License

This pallet is licensed under the same license as the Fennel Protocol. 