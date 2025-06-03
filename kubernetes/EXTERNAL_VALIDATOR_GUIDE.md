# 🌐 Fennel Network External Validator Guide

## Welcome to Fennel Network!

This guide will help you join the Fennel network as an external validator using our Docker image and connect through our public bootnodes.

## 🔄 **Two-Phase Onboarding Process**

**Phase 1: Full Node Operation** 
- Start as a **full node** (not validator yet)
- Sync with the network and prove stability
- Generate session keys and request approval

**Phase 2: Validator Conversion**
- After approval, restart with `--validator` flag
- Begin block production as an active validator

This approach ensures network security and validates your technical competency before granting validator privileges.

---

## 📋 Prerequisites

- **Docker**: Installed and running on your system
- **Hardware**: Minimum 4 CPU cores, 8GB RAM, 200GB SSD storage
- **Network**: Stable internet connection with open ports
- **Security**: Firewall configured for P2P connectivity

---

## 🔐 Phase 1: Secure Infrastructure Setup

### 1.1 Create Validator Directory Structure
```bash
# Create dedicated directories for your validator
mkdir -p ~/fennel-validator/{data,keystore,logs,scripts}
cd ~/fennel-validator

# Set secure permissions
chmod 700 keystore
chmod 755 data logs scripts
```

### 1.2 Configure Firewall (Ubuntu/Debian)
```bash
# Allow P2P port for blockchain communication
sudo ufw allow 30333/tcp

# ⚠️ RPC Security: Only allow RPC from trusted sources
# For local monitoring only:
sudo ufw allow from 127.0.0.1 to any port 9944

# For remote monitoring (replace with your monitoring server IP):
# sudo ufw allow from YOUR_MONITORING_IP to any port 9944

# For Polkadot.js Apps access (if needed):
# sudo ufw allow from YOUR_TRUSTED_IP to any port 9944

# ❌ NEVER do this (extremely dangerous):
# sudo ufw allow 9944/tcp  # This opens RPC to the entire internet!

# Enable firewall
sudo ufw enable
```

**🔒 RPC Security Note**: 
- The Docker configuration exposes RPC on `0.0.0.0:9944` for flexibility
- Always use firewall rules to restrict access to trusted IPs only
- Consider using SSH tunneling for remote access: `ssh -L 9944:localhost:9944 user@server`
- Use `--rpc-methods Safe` to disable potentially dangerous RPC methods

### 1.3 Download Fennel Docker Image
```bash
# Pull the official Fennel validator image
docker pull ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df

# Verify the image
docker image inspect ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df
```

---

## 🚀 Phase 2: Start Your Fennel Node

### 2.1 Get Current Bootnode Addresses

**Contact the Fennel team** or check the official documentation for current bootnode addresses. They will look like:
```
/ip4/BOOTNODE_IP/tcp/30333/p2p/BOOTNODE_PEER_ID
```

Example format:
```
/ip4/203.0.113.1/tcp/30333/p2p/12D3KooWExample1234567890AbCdEfGhIjKlMnOpQrStUvWxYz
/ip4/203.0.113.2/tcp/30333/p2p/12D3KooWExample0987654321ZyXwVuTsRqPoNmLkJiHgFeDcBa
```

### 2.2 Start Your Validator Node

Create a startup script:

```bash
# Create startup script
cat > ~/fennel-validator/scripts/start-validator.sh << 'EOF'
#!/bin/bash

# Fennel Validator Startup Script
# Update the BOOTNODE addresses below with current values

set -e

# Configuration
VALIDATOR_NAME="YourValidatorName"  # Change this to your validator name
NODE_KEY_FILE="$HOME/fennel-validator/data/node_key"

# Bootnode addresses - UPDATE THESE WITH CURRENT VALUES
BOOTNODES="/ip4/BOOTNODE_IP_1/tcp/30333/p2p/BOOTNODE_PEER_ID_1,/ip4/BOOTNODE_IP_2/tcp/30333/p2p/BOOTNODE_PEER_ID_2"

# Generate node key if it doesn't exist (for stable peer identity)
if [ ! -f "$NODE_KEY_FILE" ]; then
    echo "Generating new node key..."
    openssl rand -hex 32 > "$NODE_KEY_FILE"
    chmod 600 "$NODE_KEY_FILE"
fi

echo "🚀 Starting Fennel validator: $VALIDATOR_NAME"
echo "📡 Connecting to bootnodes: $BOOTNODES"

# Start the Fennel node (FIRST AS FULL NODE - no --validator flag yet)
docker run -d \
    --name fennel-node \
    --restart unless-stopped \
    -p 30333:30333 \
    -p 9944:9944 \
    -v "$HOME/fennel-validator/data:/data" \
    -v "$HOME/fennel-validator/keystore:/keystore" \
    ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df \
    --name "$VALIDATOR_NAME" \
    --base-path /data \
    --keystore-path /keystore \
    --node-key-file /data/node_key \
    --chain fennel \
    --bootnodes "$BOOTNODES" \
    --listen-addr /ip4/0.0.0.0/tcp/30333 \
    --rpc-addr 0.0.0.0:9944 \
    --rpc-methods Safe \
    --rpc-cors all \
    --rpc-max-connections 100 \
    --prometheus-external \
    --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
    --log info

echo "✅ Fennel full node started!"
echo "📊 Monitor logs with: docker logs -f fennel-node"
echo "🔍 Check sync status: curl -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}' http://localhost:9944"
echo ""
echo "⚠️  NOTE: Starting as FULL NODE first (not validator yet)"
echo "   You'll convert to validator mode after approval from Fennel team"

EOF

chmod +x ~/fennel-validator/scripts/start-validator.sh
```

### 2.3 Update Bootnode Addresses and Start

1. **Get current bootnode addresses** from Fennel team
2. **Edit the script** and replace the BOOTNODE placeholders:
   ```bash
   nano ~/fennel-validator/scripts/start-validator.sh
   # Update the BOOTNODES line with real addresses
   ```
3. **Start your validator**:
   ```bash
   ~/fennel-validator/scripts/start-validator.sh
   ```

### 2.4 Monitor Your Node

```bash
# Check if container is running
docker ps | grep fennel-node

# Monitor logs (should show syncing like standard Polkadot nodes)
docker logs -f fennel-node

# Check sync status - should show "isSyncing": false when fully synced
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9944

# Check peer connections - should show connections to other Fennel nodes
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
  http://localhost:9944

# Get node identity (useful for registration)
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_localPeerId"}' \
  http://localhost:9944
```

**Expected sync logs should look similar to:**
```
2024-01-01 12:00:00 Fennel Node
2024-01-01 12:00:00 ✌️ version 1.0.0
2024-01-01 12:00:00 📋 Chain specification: fennel
2024-01-01 12:00:00 🏷 Node name: YourValidatorName
2024-01-01 12:00:00 👤 Role: FULL
2024-01-01 12:00:00 🏷 Local node identity is: 12D3KooW...
2024-01-01 12:00:00 📦 Highest known block at #1234
2024-01-01 12:00:00 Running JSON-RPC server: addr=0.0.0.0:9944
2024-01-01 12:00:00 ⚙️ Syncing, target=#5678 (3 peers), best: #1234
```

### 2.5 Connect via Polkadot.js Apps (Optional)

You can also monitor your node using Polkadot.js Apps:

1. **Open**: [https://polkadot.js.org/apps/](https://polkadot.js.org/apps/)
2. **Click**: The logo in top-left to switch networks
3. **Toggle**: "Development" mode
4. **Enter**: `ws://127.0.0.1:9944` (or your server's IP)
5. **Connect**: Your Fennel node will appear

This gives you a visual interface to monitor blocks, extrinsics, and network status.

---

## 🔑 Phase 3: Generate Session Keys

### 3.1 Wait for Full Sync
Before generating session keys, ensure your node is fully synced:

```bash
# Check sync status (should show "isSyncing": false)
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9944
```

### 3.2 Generate Session Keys
```bash
# Generate session keys for your validator
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
  http://localhost:9944

# Save the output - you'll need this hex string for registration!
# Example output: {"jsonrpc":"2.0","id":1,"result":"0x1234567890abcdef..."}
```

**🔒 IMPORTANT**: 
- Save the session keys output securely
- These keys are unique to your validator
- You'll need them for the registration process

### 3.3 Verify Keys Are Stored
```bash
# Verify your node has the keys
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_hasSessionKeys", "params": ["YOUR_SESSION_KEYS_HEX"]}' \
  http://localhost:9944

# Should return: {"jsonrpc":"2.0","id":1,"result":true}
```

---

## 📝 Phase 4: Request Validator Registration

### 4.1 Prepare Registration Information

Collect the following information:
- **Validator Name**: Your chosen validator name
- **Node Peer ID**: Found in your node logs
- **Session Keys**: The hex string from Phase 3
- **Contact Information**: How the Fennel team can reach you
- **Server Information**: Location, specifications
- **Technical Experience**: Your blockchain/validator experience

### 4.2 Submit Registration Request

**Contact the Fennel Network team** with your registration request including:

```
Subject: Fennel Network Validator Registration Request

Validator Information:
- Name: YourValidatorName
- Peer ID: 12D3KooW... (from your node logs)
- Session Keys: 0x1234567890abcdef... (from author_rotateKeys)

Technical Information:
- Server Location: [City, Country]
- Hardware Specs: [CPU, RAM, Storage]
- Network: [Connection speed/type]

Contact Information:
- Email: your.email@example.com
- Discord/Telegram: @yourusername
- Organization (if applicable): Your Company

Experience:
- Previous validator experience: [Details]
- Technical background: [Details]
- Commitment to network: [Details]

Node Status:
- Node is fully synced: Yes/No
- Node is running stable: Yes/No
- Session keys generated: Yes
```

### 4.3 Wait for Approval

The Fennel Network team will:
1. **Review your application**
2. **Verify your node is running**
3. **Add you to the validator set** using the ValidatorManager
4. **Notify you** when you're active

---

## 🔄 Phase 5: Convert to Validator Mode (After Approval)

### 5.1 Upgrade from Full Node to Validator

Once the Fennel team approves you and adds you to the validator set, **convert your full node to validator mode**:

```bash
# Stop the full node
docker stop fennel-node
docker rm fennel-node

# Restart as validator with --validator flag
docker run -d \
    --name fennel-validator \
    --restart unless-stopped \
    -p 30333:30333 \
    -p 9944:9944 \
    -v "$HOME/fennel-validator/data:/data" \
    -v "$HOME/fennel-validator/keystore:/keystore" \
    ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df \
    --name "$VALIDATOR_NAME" \
    --validator \
    --base-path /data \
    --keystore-path /keystore \
    --node-key-file /data/node_key \
    --chain fennel \
    --bootnodes "$BOOTNODES" \
    --listen-addr /ip4/0.0.0.0/tcp/30333 \
    --rpc-addr 0.0.0.0:9944 \
    --rpc-methods Safe \
    --rpc-cors all \
    --rpc-max-connections 100 \
    --prometheus-external \
    --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
    --log info

echo "🎉 Congratulations! You're now running as a validator!"
```

**⚠️ IMPORTANT**: Only do this AFTER:
- ✅ Fennel team has approved your application
- ✅ You've been added to the ValidatorManager
- ✅ You've been notified that you're in the active validator set

---

## ✅ Phase 6: Monitor Validator Status

### 5.1 Monitor Validator Status

Once approved, monitor your validator:

```bash
# Check if you're in the active validator set
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_call", "params": ["SessionApi_validators", "0x"]}' \
  http://localhost:9944

# Monitor block production (you should see "Prepared block" messages)
docker logs -f fennel-validator | grep "Prepared block"
```

### 5.2 Maintain Your Validator

**Security Best Practices**:
- Keep your node updated with latest Docker images
- Monitor system resources and performance
- Maintain secure backup of your keystore
- Set up monitoring and alerting
- Keep your contact information updated

**Regular Maintenance**:
```bash
# Update to latest image (when announced)
docker stop fennel-validator
docker rm fennel-validator
docker pull ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df
~/fennel-validator/scripts/start-validator.sh

# Monitor logs
docker logs -f fennel-validator

# Check validator performance
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9944
```

---

## 🆘 Troubleshooting

### Common Issues

**Node not connecting to bootnodes**:
- Verify bootnode addresses are current
- Check firewall allows port 30333
- Ensure internet connectivity

**Node not syncing**:
- Check peer connections: `system_peers` RPC call
- Verify chain specification is correct
- Check disk space and system resources

**Session key errors**:
- Ensure node is fully synced before generating keys
- Verify keys are properly saved in keystore
- Contact Fennel team if registration fails

### Getting Help

- **Documentation**: Check official Fennel documentation
- **Community**: Join Fennel Discord/Telegram
- **Issues**: Report technical issues to the team
- **Support**: Contact validator support team

---

## 📞 Contact Information

- **Website**: [Your Website]
- **Discord**: [Discord Link]
- **Telegram**: [Telegram Link]
- **Email**: validators@fennel-network.com
- **GitHub**: [GitHub Repository]

---

**Welcome to the Fennel Network validator community! 🎉** 