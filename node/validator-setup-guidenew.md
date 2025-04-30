# Validator Setup Guide for Fennel Solonet

This guide outlines the process for setting up and managing validators on the Fennel Solonet chain using the validator-manager pallet. It covers both the initial hardcoded validators (Alice and Bob) and how to dynamically add additional validators (Charlie, Dave, Eve, etc.).

## Prerequisites

- Fennel Solonet node binary (built with `cargo build --release`)
- Multiple machines (or one machine running multiple nodes)
- Access to polkadot.js Apps UI

## Part 1: Setting Up Initial Hardcoded Validators

### 1. Starting the First Validator Node (Alice)

Start the first validator node (Alice) with these parameters:

```bash
./target/release/fennel-node --base-path /tmp/alice --chain local --alice --port 30333 --rpc-port 9944 --rpc-external --rpc-cors=all --rpc-methods=Unsafe --validator
```

After Alice's node starts, look for the node identity in the logs:

```
Local node identity is: 12D3KooWXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

Note this ID as you'll need it for connecting other nodes.

### 2. Starting the Second Validator Node (Bob)

Start Bob's node with similar parameters, but include Alice's node ID as a bootnode:

```bash
./target/release/fennel-node --base-path /tmp/bob --chain local --bob --port 30334 --rpc-port 9945 --rpc-external --rpc-cors=all --rpc-methods=Unsafe --validator --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/ALICE_NODE_ID
```

Replace `ALICE_NODE_ID` with the actual node ID from Alice's logs.

At this point, Alice and Bob should be running as active validators since they are hardcoded in the genesis configuration. You should see block production messages in both node logs:

```
🙌 Starting consensus session...
🎁 Prepared block for proposing...
```

## Part 2: Dynamically Adding Additional Validators

### 3. Starting Additional Nodes (Charlie, Dave, Eve)

The Fennel Solonet chain includes automatic validator detection, which means that once you connect a node with the `--validator` flag, it will be queued for addition to the validator set without requiring manual session key submission.

Start Charlie's node:

```bash
./target/release/fennel-node --base-path /tmp/charlie --chain local --name Charlie --port 30335 --rpc-port 9946 --rpc-external --rpc-cors=all --rpc-methods=Unsafe --validator --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/ALICE_NODE_ID
```

Start Dave's node:

```bash
./target/release/fennel-node --base-path /tmp/dave --chain local --name Dave --port 30336 --rpc-port 9947 --rpc-external --rpc-cors=all --rpc-methods=Unsafe --validator --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/ALICE_NODE_ID
```

Start Eve's node:

```bash
./target/release/fennel-node --base-path /tmp/eve --chain local --name Eve --port 30337 --rpc-port 9948 --rpc-external --rpc-cors=all --rpc-methods=Unsafe --validator --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/ALICE_NODE_ID
```

### 4. Monitoring Validator Addition

After starting the additional nodes with the `--validator` flag, they will be automatically queued for addition to the validator set. No manual session key generation or submission is required due to the automatic validator detection implementation in the chain spec.

1. Check if validators are in the queue:
   - Connect to the network using Polkadot.js Apps UI (http://localhost:9944)
   - Go to "Chain State" → "validatorManager" → "validatorsToAdd()"
   - This should display an array of AccountIds for the pending validators

2. Monitor session rotations:
   - "Chain State" → "session" → "currentIndex()"
   - Session rotations occur approximately every 2 minutes (10 blocks)

3. Watch for authority set changes in the node logs:
   ```
   👴 Applying authority set change scheduled at block #X
   👴 Applying GRANDPA set change to new set [(Public(...)), 1), (Public(...)), 1), ... ]
   ```

4. Once added to the validator set, the new validators will begin producing blocks. Look for these messages in their logs:
   ```
   🙌 Starting consensus session...
   🎁 Prepared block for proposing...
   ```

### 5. Understanding the Automatic Process

The Fennel Solonet's `chain_spec.rs` includes an implementation that:

1. Automatically maps validator session keys to account IDs
2. Recognizes nodes started with the `--validator` flag
3. Adds them to the validator queue when they connect
4. Activates them as validators after session rotations

## Additional Information

### Important RPC Flags Explained

- `--rpc-external`: Makes the RPC server accessible on external interfaces (not just localhost)
- `--rpc-methods=Unsafe`: Enables potentially dangerous RPC methods required for validator operations
- Note: We specifically use `--rpc-external` + `--rpc-methods=Unsafe` instead of `--unsafe-rpc-external` as this is more security-conscious

### Removing Validators

To remove a validator:

1. Go to "Developer" → "Sudo"
2. Select "validatorManager" → "removeValidator"
3. Enter the validator's address to remove
4. Submit with Alice as sudo
5. Wait for two session rotations (approximately 20 blocks)
6. Verify removal by checking "Chain State" → "session" → "validators()"

### Important Constants

- Block time: 6 seconds
- Session rotation: Every 10 blocks (1 minute)
- Authority set change: After 2 session rotations (20 blocks, ~2 minutes)

### Common Addresses

- Alice: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
- Bob: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
- Charlie: 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y
- Dave: 5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy
- Eve: 5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw

### Troubleshooting

- **LOCK File Issues**: If you see "Resource temporarily unavailable" errors related to database lock files, ensure you don't have another instance running with the same base path
- **Node Connection Issues**: Verify bootnode parameters and network connectivity
- **Validator Not Activated**: Ensure you've included the `--validator` flag and wait for multiple session rotations
- **Restarting the Network**: If restarting from a fresh state, ensure you use new base paths or delete old chain data with `rm -rf /tmp/alice /tmp/bob /tmp/charlie /tmp/dave /tmp/eve`

