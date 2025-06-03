# 🛡️ Fennel Network Validator Management Workflow

## Overview

This document outlines the workflow for the Fennel Network sudo organization to manage external validator registrations and maintain network security.

---

## 📋 Phase 1: Validator Registration Review Process

### 1.1 Registration Request Evaluation

When a validator registration request is received, evaluate:

**🔍 Technical Requirements**:
- [ ] Node is running the correct Docker image
- [ ] Node is fully synced with the network
- [ ] Session keys are properly generated
- [ ] Hardware meets minimum requirements
- [ ] Network connectivity is stable

**🔒 Security Assessment**:
- [ ] Validator identity verification
- [ ] Server location and infrastructure
- [ ] Previous validator experience
- [ ] Technical competency assessment
- [ ] Long-term commitment evaluation

**📝 Documentation Review**:
- [ ] Complete registration information provided
- [ ] Contact information is valid
- [ ] Terms of service accepted
- [ ] Compliance with network policies

### 1.2 Node Verification Process

Before approval, verify the candidate validator node:

```bash
# Connect to your Fennel network management console
kubectl port-forward -n fennel fennel-solochain-node-0 9944:9944

# Verify the candidate node is reachable and synced
# (Get their peer ID from registration request)
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
  http://localhost:9944 | jq '.result[] | select(.peerId == "CANDIDATE_PEER_ID")'

# Check their node version and chain compatibility
# (This would require connecting to their RPC if they expose it)
```

---

## ✅ Phase 2: Validator Approval Process

### 2.1 Set Session Keys for Approved Validator

Once approved, set the validator's session keys on-chain:

```bash
# Method 1: Using Polkadot.js Apps (Recommended)
# 1. Connect to ws://localhost:9944 in Polkadot.js Apps
# 2. Go to Developer > Extrinsics
# 3. Select the validator's account (or create if needed)
# 4. Call session.setKeys(keys, proof)
# 5. Use their session keys hex and proof = 0x00

# Method 2: Using polkadot-js-cli (if available)
polkadot-js-api \
  --ws ws://localhost:9944 \
  --seed "VALIDATOR_SEED_PHRASE" \
  tx.session.setKeys \
  "VALIDATOR_SESSION_KEYS_HEX" \
  "0x00"
```

### 2.2 Add Validator via ValidatorManager

Use the ValidatorManager pallet to add the approved validator:

#### **Using Polkadot.js Apps (Recommended)**:

1. **Connect to your network**:
   ```bash
   kubectl port-forward -n fennel fennel-solochain-node-0 9944:9944
   ```

2. **Open Polkadot.js Apps**: 
   - Navigate to `https://polkadot.js.org/apps/`
   - Connect to `ws://localhost:9944`

3. **Add validator via Sudo**:
   - Go to **Developer > Sudo**
   - Select `validatorManager` pallet
   - Choose `registerValidators` extrinsic
   - Enter the validator's AccountId: `["5ValidatorAccountId123..."]`
   - Submit with sudo account

#### **Using CLI (Alternative)**:

```bash
# Create a script for validator management
cat > ~/fennel-management/add-validator.sh << 'EOF'
#!/bin/bash

# Add validator to Fennel network via ValidatorManager
# Usage: ./add-validator.sh VALIDATOR_ACCOUNT_ID

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 VALIDATOR_ACCOUNT_ID"
    echo "Example: $0 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    exit 1
fi

VALIDATOR_ID="$1"
echo "🔄 Adding validator: $VALIDATOR_ID"

# Add validator using polkadot-js-cli (requires setup)
polkadot-js-api \
  --ws ws://localhost:9944 \
  --sudo \
  --seed "YOUR_SUDO_SEED_PHRASE" \
  tx.sudo.sudo \
  tx.validatorManager.registerValidators \
  "[$VALIDATOR_ID]"

echo "✅ Validator addition submitted!"
echo "⏳ Wait for session rotation to see validator in active set"
EOF

chmod +x ~/fennel-management/add-validator.sh
```

### 2.3 Monitor Session Rotation

After adding a validator, monitor for session rotation:

```bash
# Check current validators
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_call", "params": ["SessionApi_validators", "0x"]}' \
  http://localhost:9944

# Check validators to be added
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_getStorage", "params": ["0x99971b5749ac43e0235e41b0d37869880f2b4c47b73c7f3b79b13e0e98b2e04663ba2e8c5fc4683fd10c8978ad0e6a2bf6e49026eba3b35cf71e21bb3b3f3f12"]}' \
  http://localhost:9944

# Monitor logs for session changes
kubectl logs -n fennel -l app.kubernetes.io/instance=fennel-solochain -f | grep -i session
```

---

## ❌ Phase 3: Validator Removal Process

### 3.1 Reasons for Removal

Validators may be removed for:
- **Performance issues**: Consistent missed blocks
- **Security violations**: Compromised nodes or keys
- **Policy violations**: Not following network guidelines
- **Technical issues**: Prolonged downtime or sync problems
- **Voluntary exit**: Validator request to leave

### 3.2 Removal Procedure

#### **Using Polkadot.js Apps**:

1. **Connect to network**:
   ```bash
   kubectl port-forward -n fennel fennel-solochain-node-0 9944:9944
   ```

2. **Remove via Sudo**:
   - Go to **Developer > Sudo**
   - Select `validatorManager` pallet  
   - Choose `removeValidator` extrinsic
   - Enter the validator's AccountId
   - Submit with sudo account

#### **Using CLI**:

```bash
# Create removal script
cat > ~/fennel-management/remove-validator.sh << 'EOF'
#!/bin/bash

# Remove validator from Fennel network
# Usage: ./remove-validator.sh VALIDATOR_ACCOUNT_ID REASON

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 VALIDATOR_ACCOUNT_ID [REASON]"
    exit 1
fi

VALIDATOR_ID="$1"
REASON="${2:-Manual removal}"

echo "⚠️  Removing validator: $VALIDATOR_ID"
echo "📝 Reason: $REASON"

read -p "Are you sure? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Removal cancelled"
    exit 1
fi

# Remove validator
polkadot-js-api \
  --ws ws://localhost:9944 \
  --sudo \
  --seed "YOUR_SUDO_SEED_PHRASE" \
  tx.sudo.sudo \
  tx.validatorManager.removeValidator \
  "$VALIDATOR_ID"

echo "✅ Validator removal submitted!"
echo "📝 Log this action with reason: $REASON"
EOF

chmod +x ~/fennel-management/remove-validator.sh
```

---

## 📊 Phase 4: Ongoing Validator Monitoring

### 4.1 Regular Health Checks

Set up monitoring for all validators:

```bash
# Create monitoring script
cat > ~/fennel-management/monitor-validators.sh << 'EOF'
#!/bin/bash

# Monitor Fennel network validators
echo "📊 Fennel Network Validator Status"
echo "=================================="

# Get current validators
echo "🔍 Current Active Validators:"
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_call", "params": ["SessionApi_validators", "0x"]}' \
  http://localhost:9944 | jq -r '.result'

echo ""
echo "⏳ Validators Pending Addition:"
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_getStorage", "params": ["0x99971b5749ac43e0235e41b0d37869880f2b4c47b73c7f3b79b13e0e98b2e04663ba2e8c5fc4683fd10c8978ad0e6a2bf6e49026eba3b35cf71e21bb3b3f3f12"]}' \
  http://localhost:9944 | jq -r '.result'

echo ""
echo "🗑️  Validators Pending Removal:"
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_getStorage", "params": ["0x99971b5749ac43e0235e41b0d37869880f2b4c47b73c7f3b79b13e0e98b2e046638aadfe6c6d08db46efe4a535b7cc47a7c8e5b6c3db14f6c29f2b6c3a5c3"]}' \
  http://localhost:9944 | jq -r '.result'

echo ""
echo "🏗️  Current Block Height:"
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
  http://localhost:9944 | jq -r '.result.number'

echo ""
echo "🌐 Network Health:"
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9944 | jq '.'
EOF

chmod +x ~/fennel-management/monitor-validators.sh
```

### 4.2 Automated Alerts (Optional)

Set up automated monitoring:

```bash
# Create alerting script
cat > ~/fennel-management/check-validator-performance.sh << 'EOF'
#!/bin/bash

# Check validator performance and send alerts
# Run this via cron every 10 minutes

ALERT_EMAIL="admin@fennel-network.com"
ALERT_THRESHOLD=10  # Alert if no blocks in last 10 minutes

# Check recent block production
RECENT_BLOCKS=$(kubectl logs -n fennel --tail=100 -l app.kubernetes.io/instance=fennel-solochain | grep "Prepared block" | tail -5)

if [ -z "$RECENT_BLOCKS" ]; then
    echo "⚠️  ALERT: No recent block production detected" | mail -s "Fennel Network Alert" $ALERT_EMAIL
fi

# Check node connectivity
PEER_COUNT=$(curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
  http://localhost:9944 | jq '.result | length')

if [ "$PEER_COUNT" -lt 2 ]; then
    echo "⚠️  ALERT: Low peer count: $PEER_COUNT" | mail -s "Fennel Network Alert" $ALERT_EMAIL
fi
EOF

chmod +x ~/fennel-management/check-validator-performance.sh

# Add to cron (optional)
# */10 * * * * /path/to/check-validator-performance.sh
```

---

## 📚 Phase 5: Documentation and Compliance

### 5.1 Validator Registry

Maintain a registry of all validators:

```bash
# Create validator registry
cat > ~/fennel-management/validator-registry.json << 'EOF'
{
  "fennelValidators": {
    "lastUpdated": "2024-01-01T00:00:00Z",
    "totalValidators": 0,
    "validators": [
      {
        "accountId": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "name": "Alice (Genesis)",
        "status": "active",
        "addedDate": "2024-01-01",
        "contact": "genesis@fennel.com",
        "location": "Genesis",
        "nodeVersion": "1.0.0"
      }
    ]
  }
}
EOF
```

### 5.2 Change Log

Document all validator changes:

```bash
# Create change log template
cat > ~/fennel-management/validator-changes.log << 'EOF'
# Fennel Network Validator Change Log
# Format: [YYYY-MM-DD HH:MM] ACTION VALIDATOR_ID REASON

[2024-01-01 00:00] GENESIS 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY Initial validator
[2024-01-01 00:00] GENESIS 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty Initial validator
EOF
```

---

## 🔧 Tools and Scripts Summary

All management scripts are located in `~/fennel-management/`:

- `add-validator.sh` - Add approved validators
- `remove-validator.sh` - Remove validators
- `monitor-validators.sh` - Check validator status
- `check-validator-performance.sh` - Automated monitoring
- `validator-registry.json` - Validator database
- `validator-changes.log` - Change history

---

## 🛡️ Security Best Practices

1. **Sudo Key Security**:
   - Store sudo keys in HSM or air-gapped systems
   - Use multi-sig for critical operations
   - Rotate keys regularly

2. **Validator Verification**:
   - Always verify validator nodes before approval
   - Check technical competency
   - Validate long-term commitment

3. **Monitoring**:
   - Implement comprehensive monitoring
   - Set up automated alerts
   - Regular performance reviews

4. **Documentation**:
   - Maintain detailed records
   - Log all changes with reasons
   - Regular security audits

---

**This workflow ensures secure and efficient management of the Fennel validator network! 🛡️** 