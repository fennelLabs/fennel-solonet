# External Validator Setup Guide

This guide explains how to properly add external validators to the Fennel blockchain network.

## Prerequisites

- Running Fennel blockchain network
- Access to a sudo account (for adding validators)
- External validator node with:
  - Fennel node binary
  - Network connectivity to existing nodes
  - Proper firewall configuration (port 30333 for P2P)

## Process Overview

1. **Validator sets up their node** using FennelValidator scripts
2. **Validator generates session keys** on their node
3. **Validator sends keys and stash account** to sudo/admin
4. **Sudo funds the validator's stash account**
5. **Sudo registers the session keys** on behalf of the validator
6. **Sudo adds the validator** via validator-manager pallet
7. **Wait for session rotation** (2 sessions)
8. **Validator becomes active**

## Step 1: Validator Node Setup

The validator uses the FennelValidator repository to set up their node:

```bash
# Clone and setup
git clone https://github.com/CorruptedAesthetic/FennelValidator.git
cd FennelValidator
./install.sh
./setup-validator.sh
```

## Step 2: Generate Session Keys (Validator)

The validator generates their session keys using the provided script:

```bash
# This temporarily enables unsafe RPC to generate keys
./tools/internal/generate-keys-with-restart.sh

# Or manually via RPC (if node is running with unsafe RPC)
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9944
```

This generates:
- Combined session keys (Aura + GRANDPA concatenated)
- Individual Aura key (sr25519)
- Individual GRANDPA key (ed25519)

The keys are saved to `validator-data/session-keys.json`.

## Step 3: Submit Information to Sudo (Validator)

The validator runs the registration completion script:

```bash
./tools/complete-registration.sh
```

This generates a submission file containing:
- Validator name
- Stash account address
- Controller account address (if different)
- Session keys
- Node information

The validator sends this file to the sudo/admin.

## Step 4: Fund Validator Account (Sudo)

Using Polkadot.js [[memory:2304110]] or similar tool, sudo transfers funds to the validator's stash account:

```javascript
// Transfer funds to stash account
const tx = api.tx.balances.transfer(validatorStashAccount, amount);
await tx.signAndSend(sudoAccount);
```

## Step 5: Register Session Keys (Sudo)

Sudo registers the validator's session keys on their behalf:

```javascript
// Using Polkadot.js API
const keys = '0x...'; // Session keys from validator
const proof = '0x00'; // Empty proof

// Set keys on behalf of the validator
const tx = api.tx.sudo.sudoAs(
  validatorAccount,
  api.tx.session.setKeys(keys, proof)
);
await tx.signAndSend(sudoAccount);
```

## Step 6: Add Validator (Sudo)

After keys are registered, sudo adds the validator:

```javascript
// Add validator to the set
const tx = api.tx.validatorManager.registerValidators([validatorAccount]);
await tx.signAndSend(sudoAccount);
```

**Important**: The validator-manager pallet now checks that session keys are registered before adding validators. If keys are not registered, the transaction will fail with `NoKeysRegistered` error.

## Step 7: Verify and Monitor

### Check Registration Status

1. **Verify session keys are registered:**
   ```javascript
   const keys = await api.query.session.nextKeys(validatorAccount);
   console.log('Keys registered:', !keys.isEmpty);
   ```

2. **Check pending validators:**
   ```javascript
   const toAdd = await api.query.validatorManager.validatorsToAdd();
   console.log('Validators to be added:', toAdd.toJSON());
   ```

3. **Monitor validator logs:**
   ```bash
   # On validator node
   tail -f data/chains/*/network.log | grep -E "(GRANDPA|Aura|authority)"
   ```

4. **Check active validators after session rotation:**
   ```javascript
   const validators = await api.query.session.validators();
   console.log('Active validators:', validators.toJSON());
   ```

## Troubleshooting

### Common Errors

1. **`NoKeysRegistered` error when adding validator**
   - Cause: Session keys not registered before calling `registerValidators`
   - Fix: Ensure `session.setKeys()` is called first

2. **GRANDPA stalls after adding validator**
   - Cause: Validator added without proper session keys
   - Fix: This should no longer happen with the updated pallet

3. **Validator not producing blocks**
   - Check if keys are properly inserted in keystore
   - Verify network connectivity
   - Ensure node is fully synced

### Debugging Commands

```bash
# Check if validator is connected to peers
curl http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers", "params":[]}'

# Check sync status
curl http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState", "params":[]}'

# Check if node has the correct role
curl http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_nodeRoles", "params":[]}'
```

## Security Considerations

1. **Key Management**
   - Validators generate and control their own keys
   - Keys should be backed up securely
   - Never share private keys or seed phrases

2. **Communication Security**
   - Use secure channels when sending registration info to sudo
   - Verify validator identity before processing registration

3. **Operational Security**
   - Monitor validator performance
   - Set up alerts for validator downtime
   - Keep node software updated

## Session Timing

- **Session duration**: 25 blocks (5 minutes with 12-second blocks)
- **Validator activation**: Current session + 2
- **Total time to activation**: ~10-15 minutes after registration

## Best Practices

1. **Pre-flight Checks**
   - Ensure node is fully synced before generating keys
   - Verify network connectivity
   - Test on staging network first

2. **Key Backup**
   - Backup seed phrases securely
   - Store network keys separately
   - Document key derivation paths

3. **Monitoring**
   - Set up Prometheus metrics
   - Monitor block production
   - Track GRANDPA finalization

4. **Updates**
   - Coordinate runtime upgrades
   - Test updates on staging first
   - Have rollback plan ready 