# Quick Guide: Adding Validators to Fennel

## Step 1: Scale the Kubernetes Deployment
```bash
# Add 3 more validators (total 5)
kubectl scale statefulset fennel-solochain-node -n fennel --replicas=5
```

## Step 2: Port-forward to New Nodes
```bash
# Terminal 1 (already have node-0 on 9944)
# Terminal 2
kubectl port-forward -n fennel fennel-solochain-node-2 9946:9944
# Terminal 3
kubectl port-forward -n fennel fennel-solochain-node-3 9947:9944
# Terminal 4
kubectl port-forward -n fennel fennel-solochain-node-4 9948:9944
```

## Step 3: Generate Session Keys for Each New Validator
```bash
# Node 2
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9946

# Node 3
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9947

# Node 4
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9948
```

## Step 4: In Polkadot.js Apps (ws://localhost:9944)

### Set Session Keys:
1. Go to **Developer > Extrinsics**
2. For each new validator:
   - Select account (create if needed)
   - `session.setKeys(keys, proof)`
   - Paste the hex keys from Step 3
   - Set proof as `0x00`
   - Submit

### Register with Validator Manager:
1. Go to **Developer > Sudo**
2. Select `validatorManager.registerValidators`
3. Enter array of AccountIds:
   ```
   ["AccountId1", "AccountId2", "AccountId3"]
   ```
4. Submit with sudo account

## Step 5: Wait for Session Rotation
- Check `Developer > Chain State > session.validators()`
- New validators appear after 1-2 sessions

---

# Removing Validators

## In Polkadot.js Apps:
1. Go to **Developer > Sudo**
2. Select `validatorManager.removeValidator`
3. Enter the AccountId to remove
4. Submit with sudo

## Scale Down Kubernetes (Optional):
```bash
# If permanently removing nodes
kubectl scale statefulset fennel-solochain-node -n fennel --replicas=2
``` 