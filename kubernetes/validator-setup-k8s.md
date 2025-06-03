# Fennel Validator Setup in Kubernetes

This guide follows the same validator management process as your local testnet, adapted for Kubernetes.

## Overview

Your Fennel blockchain uses:
- **Session Pallet**: Manages validator session keys
- **Validator Manager Pallet**: Sudo-controlled validator registration
- **Initial Validators**: Defined in chainspec (like Alice and Bob)

## Initial Deployment

The deployment creates 2 validator nodes by default (similar to Alice and Bob in your testnet):

```bash
cd kubernetes
./deploy-fennel.sh
```

## Validator Setup Process

### 1. Check Deployment Status

```bash
# View pods
kubectl get pods -n fennel

# Expected output:
# NAME                       READY   STATUS    RESTARTS   AGE
# fennel-solochain-node-0    1/1     Running   0          5m
# fennel-solochain-node-1    1/1     Running   0          5m
```

### 2. Access RPC Endpoints

Set up port-forwarding to access each validator's RPC:

```bash
# Terminal 1 - Node 0 (like Alice)
kubectl port-forward -n fennel fennel-solochain-node-0 9944:9944

# Terminal 2 - Node 1 (like Bob)
kubectl port-forward -n fennel fennel-solochain-node-1 9945:9944
```

### 3. Generate Session Keys

For each validator node, generate session keys:

```bash
# For node-0
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9944

# For node-1 (using port 9945)
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9945
```

Save the returned hex strings (e.g., `0x...`).

### 4. Connect Polkadot.js Apps

1. Open [Polkadot.js Apps](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944)
2. You should see the initial validators from your chainspec

### 5. Set Session Keys On-Chain

For each validator:
1. Go to **Developer → Extrinsics**
2. Select the validator's account as sender
3. Choose `session` → `setKeys`
4. Paste the session keys hex string
5. Set proof as `0x`
6. Submit transaction

### 6. Register Additional Validators

To add more validators (like adding Charlie, Dave, Eve):

#### Scale the StatefulSet:
```bash
kubectl scale statefulset fennel-solochain-node -n fennel --replicas=5
```

#### Wait for new nodes to sync:
```bash
kubectl logs -n fennel fennel-solochain-node-2 -f
```

#### Generate keys for new validators:
```bash
# Port forward for node-2
kubectl port-forward -n fennel fennel-solochain-node-2 9946:9944

# Generate keys
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9946
```

#### Register with Validator Manager:
1. In Polkadot.js Apps, go to **Developer → Sudo**
2. Select `validatorManager` → `registerValidators`
3. Enter AccountIds as array (get these from your chainspec or generate new accounts)
4. Submit with sudo account

### 7. Monitor Validators

Check active validators:
```bash
# Via Polkadot.js Apps
# Developer → Chain State → session → validators()

# Check logs for block production
kubectl logs -n fennel -l app.kubernetes.io/instance=fennel-solochain -f | grep "Prepared block"
```

## Key Differences from Local Testnet

1. **Node Names**: Instead of Alice/Bob/Charlie, nodes are named `fennel-solochain-node-0`, `-1`, etc.
2. **Bootnodes**: Kubernetes handles peer discovery via headless service
3. **Persistence**: Each node has its own persistent volume
4. **Access**: Use port-forwarding instead of direct localhost ports

## Troubleshooting

### Nodes Not Finding Peers
```bash
# Check service
kubectl get svc -n fennel

# Check node logs
kubectl logs -n fennel fennel-solochain-node-0 | grep "Discovered"
```

### Session Keys Not Working
```bash
# Verify keys in keystore
kubectl exec -n fennel fennel-solochain-node-0 -- ls /keystore
```

### Database Issues
```bash
# If you need to purge and restart (like rm -rf /tmp/alice)
kubectl delete pvc -n fennel --all
kubectl delete pods -n fennel --all
```

## Adding More Validators

1. Scale the StatefulSet
2. Generate session keys for new nodes
3. Set keys on-chain
4. Register with validator manager using sudo
5. Wait for session rotation

The process is identical to your local testnet, just using Kubernetes resources instead of local directories! 