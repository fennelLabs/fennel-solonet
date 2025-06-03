# 🧪 Fennel Network External Validator Testing Strategy

## 📊 **Current Network State**

### **Active Validators:**
- **Alice**: `12D3KooWR6LStFm9Vif78LEVuDRE9tYA2zJ8r4qTENoQKmv4tA5h` ✅ Producing blocks
- **Bob**: `12D3KooWH836DFpUGv6FedorW8hUbmadYFJKQuX5qLeUNYzRYieN` ✅ Connected to Alice

### **Network Health:**
- ✅ **P2P Connection**: Alice ↔ Bob (within Kubernetes cluster)
- ✅ **Block Production**: Chain actively producing blocks  
- ✅ **ValidatorManager**: Operational and managing validator set
- ✅ **RPC Access**: Available via port-forward on localhost:9944

---

## 🎯 **Testing Strategy Overview**

### **Phase 1: Bootnode Testing (Network Discovery)**
**Goal**: Ensure external validators can discover Alice & Bob through bootnodes

**Actors**:
- **Alice & Bob**: Existing validators (keep running)
- **Bootnodes**: New public discovery nodes
- **External nodes**: Test connections through bootnodes

### **Phase 2: External Validator Testing (Registration Process)**
**Goal**: Validate complete external validator onboarding workflow

**Test Subjects**:
- **Charlie**: `5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y`
- **Dave**: `5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy`
- **Eve**: `5HGjWAeFDfFCWPsjFQdVV2Mspz2XtMktvgocEZcCj68kUMaw`

---

## 🧪 **Detailed Testing Plan**

### **Test 1: Bootnode Connectivity**

#### **1.1 Run Bootnode Connectivity Test**
```bash
cd /home/neurosx/WORKING_WORKSPACE/fennel-solonet-2503UPGRADE/kubernetes
./test-bootnode-connectivity.sh
```

**What it tests**:
- ✅ Current Alice/Bob P2P connection
- ✅ Bootnode deployment process
- ✅ Bootnode discovery of Alice/Bob
- ✅ External node connection through bootnodes

#### **1.2 Expected Results**
- **Before bootnodes**: Alice sees only Bob as peer
- **After bootnodes**: Alice sees Bob + 2 bootnodes as peers
- **Bootnode logs**: Show connections to Alice & Bob
- **External test**: Can discover network through bootnodes

### **Test 2: External Validator Simulation**

#### **2.1 Run Charlie/Dave/Eve Test**
```bash
cd /home/neurosx/WORKING_WORKSPACE/fennel-solonet-2503UPGRADE/kubernetes
./test-charlie-dave-eve.sh --test-all
```

**What it tests**:
- 🚀 **Node Startup**: Can Charlie/Dave/Eve start as full nodes?
- 🔑 **Session Keys**: Can they generate session keys?
- 📡 **Network Sync**: Can they sync with Alice/Bob's chain?
- 🔗 **P2P Discovery**: Can they find peers through bootnodes?

#### **2.2 Expected Results**
```
👤 Charlie:
  💳 Account ID: 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y
  🆔 Peer ID: 12D3KooW... (unique)
  🔑 Session Keys: 0x1234... (generated)
  🌐 RPC Port: 9946

👤 Dave:
  💳 Account ID: 5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy
  🆔 Peer ID: 12D3KooW... (unique)
  🔑 Session Keys: 0x5678... (generated)
  🌐 RPC Port: 9947

👤 Eve:
  💳 Account ID: 5HGjWAeFDfFCWPsjFQdVV2Mspz2XtMktvgocEZcCj68kUMaw
  🆔 Peer ID: 12D3KooW... (unique)
  🔑 Session Keys: 0x9abc... (generated)
  🌐 RPC Port: 9948
```

### **Test 3: ValidatorManager Registration**

#### **3.1 Test Session Key Setup**
Once Charlie/Dave/Eve are running with session keys:

1. **Connect to Alice**: `kubectl port-forward -n fennel fennel-solochain-node-0 9944:9944`
2. **Open Polkadot.js Apps**: Connect to `ws://localhost:9944`
3. **For each test validator**:
   - Go to **Developer > Extrinsics**
   - Select validator's account as sender
   - Call `session.setKeys(session_keys, 0x00)`
   - Submit transaction

#### **3.2 Test ValidatorManager Addition**
1. **Use Sudo to add validators**:
   - Go to **Developer > Sudo**
   - Call `validatorManager.registerValidators([charlie_account])`
   - Submit with sudo account
   - Repeat for Dave and Eve

#### **3.3 Monitor Session Rotation**
```bash
# Watch for session changes
kubectl logs -n fennel -l app.kubernetes.io/instance=fennel-solochain -f | grep -i session

# Check active validator set
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_call", "params": ["SessionApi_validators", "0x"]}' \
  http://localhost:9944
```

#### **3.4 Test Validator Mode Conversion**
Once added to ValidatorManager, convert test nodes to validator mode:
```bash
# Stop Charlie's full node
docker stop fennel-test-charlie

# Restart as validator (add --validator flag)
docker run -d --name fennel-test-charlie-validator \
  -p 9946:9944 -v /tmp/fennel-test-charlie:/data \
  ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df \
  --validator --name "TestCharlie" --base-path /data --chain fennel \
  --rpc-addr 0.0.0.0:9944 --rpc-methods Safe

# Check for block production
docker logs -f fennel-test-charlie-validator | grep "Prepared block"
```

---

## 🎯 **Success Criteria**

### **Bootnode Tests ✅**
- [ ] Bootnodes deploy successfully with public IPs
- [ ] Bootnodes connect to Alice and Bob
- [ ] Alice/Bob see bootnodes as peers
- [ ] External nodes can discover network via bootnodes

### **External Validator Tests ✅**
- [ ] Charlie/Dave/Eve start as full nodes
- [ ] All generate unique session keys
- [ ] All can connect to the network
- [ ] All can sync with current chain state

### **ValidatorManager Tests ✅**
- [ ] Session keys set successfully for test validators
- [ ] ValidatorManager accepts new validators
- [ ] Session rotation includes new validators
- [ ] New validators can produce blocks in validator mode

### **Network Stability ✅**
- [ ] Alice and Bob remain stable throughout testing
- [ ] Block production continues uninterrupted
- [ ] P2P network scales properly with new nodes
- [ ] ValidatorManager functions correctly under load

---

## 🔧 **Testing Tools Available**

### **Scripts Created:**
```bash
# Test bootnode deployment and connectivity
./test-bootnode-connectivity.sh

# Deploy public bootnodes  
./deploy-bootnodes.sh

# Get bootnode addresses for external validators
./get-bootnode-addresses.sh

# Test Charlie/Dave/Eve as external validators
./test-charlie-dave-eve.sh

# Interactive testing menu
./test-charlie-dave-eve.sh  # (no args for menu)

# Quick full test
./test-charlie-dave-eve.sh --test-all

# Cleanup test environments
./test-charlie-dave-eve.sh --cleanup
```

### **Monitoring Commands:**
```bash
# Check Alice's current state
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
  http://localhost:9944

# Monitor block production
kubectl logs -n fennel -l app.kubernetes.io/instance=fennel-solochain -f | grep "Prepared block"

# Check validator set
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_call", "params": ["SessionApi_validators", "0x"]}' \
  http://localhost:9944
```

---

## 🚀 **Quick Start Testing**

### **Option 1: Full Automated Test**
```bash
cd /home/neurosx/WORKING_WORKSPACE/fennel-solonet-2503UPGRADE/kubernetes

# Test bootnode connectivity
./test-bootnode-connectivity.sh

# Test external validators
./test-charlie-dave-eve.sh --test-all
```

### **Option 2: Interactive Testing**
```bash
cd /home/neurosx/WORKING_WORKSPACE/fennel-solonet-2503UPGRADE/kubernetes

# Interactive bootnode testing
./test-bootnode-connectivity.sh

# Interactive external validator testing
./test-charlie-dave-eve.sh
```

### **Option 3: Manual Step-by-Step**
1. Deploy bootnodes: `./deploy-bootnodes.sh`
2. Get bootnode addresses: `./get-bootnode-addresses.sh`
3. Start Charlie: `./test-charlie-dave-eve.sh` → option 2 → Charlie → start
4. Generate keys: `./test-charlie-dave-eve.sh` → option 4
5. Use Polkadot.js Apps to add Charlie via ValidatorManager
6. Repeat for Dave and Eve

---

## 📊 **Testing Timeline**

### **Phase 1: Infrastructure (30 minutes)**
- ✅ Verify Alice/Bob stability
- 🚀 Deploy and test bootnodes
- 🔗 Verify P2P connectivity

### **Phase 2: External Nodes (20 minutes)**  
- 🚀 Start Charlie/Dave/Eve as full nodes
- 🔑 Generate session keys for all
- 📡 Test network discovery and sync

### **Phase 3: Validator Onboarding (30 minutes)**
- 🔧 Set session keys via Polkadot.js Apps
- ✅ Add validators via ValidatorManager
- 🔄 Monitor session rotation

### **Phase 4: Validation (15 minutes)**
- 🎯 Convert to validator mode
- 📊 Verify block production
- 🌐 Test complete network health

**Total Testing Time: ~95 minutes**

---

## 🎉 **Expected Final State**

After successful testing:

```
🌐 Fennel Network Status:
- Alice: ✅ Validator (original)
- Bob: ✅ Validator (original)  
- Charlie: ✅ Validator (added via ValidatorManager)
- Dave: ✅ Validator (added via ValidatorManager)
- Eve: ✅ Validator (added via ValidatorManager)
- Bootnodes: ✅ 2 public discovery nodes

📊 Network Metrics:
- Total Validators: 5
- Block Production: All validators participating
- P2P Network: Fully meshed via bootnodes
- ValidatorManager: Tested add/remove functionality
```

**🎯 Result**: Complete validation of external validator onboarding process and ValidatorManager functionality! 🚀 