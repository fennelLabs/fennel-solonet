# Fennel Validator Management Guide

This guide covers validator management for the Fennel proof-of-authority solochain with custom validator pallet.

## Overview

Fennel uses:
- **Session Pallet**: For managing validator session keys
- **Custom Validator Pallet**: For sudo-controlled validator onboarding/offboarding
- **Aura**: For block production consensus
- **GRANDPA**: For block finalization

## Initial Setup

### 1. Deploy Initial Validators

The deployment creates 3 validator nodes by default. Each node automatically generates session keys on first start.

### 2. Retrieve Session Keys

For each validator node, retrieve the session keys:

```bash
# For node 0
kubectl exec -n fennel fennel-solochain-node-0 -- \
  curl -s -H 'Content-Type: application/json' \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
  http://localhost:9944 | jq -r '.result'

# For node 1
kubectl exec -n fennel fennel-solochain-node-1 -- \
  curl -s -H 'Content-Type: application/json' \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
  http://localhost:9944 | jq -r '.result'

# For node 2
kubectl exec -n fennel fennel-solochain-node-2 -- \
  curl -s -H 'Content-Type: application/json' \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
  http://localhost:9944 | jq -r '.result'
```

Save these keys - you'll need them for validator registration.

### 3. Set Session Keys

Using Polkadot.js Apps or similar tool:

1. Connect to your RPC endpoint
2. Go to Developer > Extrinsics
3. For each validator:
   - Select the validator's account
   - Call `session.setKeys(keys, proof)`
   - Where `keys` is the output from `author_rotateKeys`
   - `proof` can be `0x00`

## Validator Onboarding (via Custom Pallet)

Since Fennel uses a custom validator pallet with sudo control:

### Using Polkadot.js Apps

1. Connect to RPC endpoint:
   ```bash
   kubectl port-forward -n fennel svc/fennel-solochain-node 9944:9944
   ```
   Then connect to `ws://localhost:9944`

2. Navigate to Developer > Sudo

3. Select your custom validator pallet method for adding validators

4. Submit the sudo transaction with:
   - Validator account address
   - Any additional parameters your pallet requires

### Using CLI (example)

```bash
# Port forward first
kubectl port-forward -n fennel svc/fennel-solochain-node 9944:9944 &

# Use polkadot-js-api or similar CLI tool
# Example structure (adjust based on your pallet):
polkadot-js-api \
  --ws ws://localhost:9944 \
  --sudo \
  --seed "YOUR_SUDO_SEED" \
  tx.sudo.sudo \
  tx.validatorPallet.addValidator "VALIDATOR_ADDRESS"
```

## Validator Offboarding

Similar process using your custom pallet's remove/offboard method:

1. Via Polkadot.js Apps:
   - Developer > Sudo
   - Select validator removal method
   - Submit with validator address

2. The validator will be removed in the next session

## Adding New Validator Nodes

To add more validator nodes to your Kubernetes cluster:

### 1. Scale the StatefulSet

```bash
# Scale to 5 validators
kubectl scale statefulset fennel-solochain-node -n fennel --replicas=5
```

### 2. Wait for New Nodes to Sync

```bash
# Check sync status
kubectl logs -n fennel fennel-solochain-node-3 -f
kubectl logs -n fennel fennel-solochain-node-4 -f
```

### 3. Get Session Keys from New Nodes

```bash
# For new node 3
kubectl exec -n fennel fennel-solochain-node-3 -- \
  curl -s -H 'Content-Type: application/json' \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
  http://localhost:9944 | jq -r '.result'
```

### 4. Register New Validators

Follow the onboarding process above for the new validators.

## Monitoring Validators

### Check Current Validators

```bash
# Via RPC
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
  http://localhost:9944

# Check session info
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_getMetadata"}' \
  http://localhost:9944
```

### Monitor Block Production

```bash
# Watch logs for block production
kubectl logs -n fennel -l app.kubernetes.io/instance=fennel-solochain -f | grep "Prepared block"
```

### Prometheus Metrics

If you have Prometheus configured:
- `substrate_block_height` - Current block height
- `substrate_finality_grandpa_round` - GRANDPA round number
- `substrate_sub_libp2p_peers_count` - Number of connected peers

## Key Rotation

For security, periodically rotate validator session keys:

1. Generate new keys:
   ```bash
   kubectl exec -n fennel fennel-solochain-node-0 -- \
     curl -s -H 'Content-Type: application/json' \
     -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
     http://localhost:9944
   ```

2. Update keys on-chain via `session.setKeys()`

3. Keys will be active in the next session

## Backup and Recovery

### Backup Validator Keys

```bash
# Create backup of keystore
kubectl cp fennel/fennel-solochain-node-0:/keystore ./validator-0-keystore-backup
```

### Restore Validator Keys

1. Stop the validator
2. Restore keystore to the persistent volume
3. Restart the validator

## Troubleshooting

### Validator Not Producing Blocks

1. Check if validator is in the active set
2. Verify session keys are correctly set
3. Check node is fully synced
4. Review logs for errors:
   ```bash
   kubectl logs -n fennel fennel-solochain-node-0 | grep -E "ERROR|WARN"
   ```

### Session Key Mismatch

If you see "Session key not found" errors:
1. Regenerate keys with `author_rotateKeys`
2. Update on-chain with `session.setKeys`
3. Wait for next session

### Network Connectivity Issues

1. Check P2P service is accessible:
   ```bash
   kubectl get svc -n fennel
   ```
2. Verify peers are connected:
   ```bash
   kubectl exec -n fennel fennel-solochain-node-0 -- \
     curl -s -H 'Content-Type: application/json' \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
     http://localhost:9944
   ``` 