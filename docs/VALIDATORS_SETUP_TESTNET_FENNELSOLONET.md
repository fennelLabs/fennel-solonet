# Validator Management Instructions

## Starting a 5-Node Testnet (Alice, Bob, Charlie, Dave, Eve)

### 1. Build the Node Binary
```bash
cd /home/neurosx/WORKING_WORKSPACE/fennel-solonet-2503UPGRADE
cargo build --release
```

### 2. Purge Old Chain Data (Recommended for a Fresh Start)
```bash
rm -rf /tmp/alice /tmp/bob /tmp/charlie /tmp/dave /tmp/eve
```

### 3. Start Alice (Terminal 1)
```bash
./run_alice.sh
```
- Note Alice's node ID from the logs (e.g., `12D3KooW...`).

### 4. Start Bob (Terminal 2)
```bash
./run_bob.sh
```
- Bob's script should already point to Alice's node ID in the `--bootnodes` flag.

### 5. Start Charlie, Dave, and Eve (Terminals 3, 4, 5)
```bash
./run_charlie.sh
./run_dave.sh
./run_eve.sh
```
- Ensure each script includes Alice's node ID in the `--bootnodes` flag.

### 6. Connect Polkadot.js Apps
- Open [https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944)
- Confirm you see Alice and Bob as initial validators in `session.validators()`.

### 7. Dynamically Add Charlie, Dave, and Eve as Validators
- Follow the steps below (Generate/Set Session Keys, Register Validators, etc.) to add them to the active set.

---

## 1. Prerequisites
- Your chain is running with Alice and Bob as initial validators.
- You have built and launched nodes for Charlie, Dave, and Eve with the `--validator` flag and correct `--bootnodes` (pointing to Alice).
- You have access to Polkadot.js Apps connected to your local node.

---

## 2. Generate and Set Session Keys for New Validators

**For each new validator (Charlie, Dave, Eve):**

### a. Generate Session Keys
- On the validator node (e.g., Charlie), run:
  ```bash
  curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:<RPC_PORT>
  ```
  - Replace `<RPC_PORT>` with the node's RPC port (e.g., 9946 for Charlie).
  - Copy the returned hex string (e.g., `0x...`).

  Alternatively: 

Connect Polkadot.js Apps to Charlie's node (e.g., ws://127.0.0.1:9946).
In Polkadot.js Apps, go to Developer → RPC calls and use author_rotateKeys() to generate new session keys on Charlie's node.
This returns a hex string (e.g., 0x...), which is the public part of the new session keys.
Copy the returned hex string.

### b. Set Session Keys On-Chain
- go on alice's node (boot) 
- In Polkadot.js Apps, go to **Developer → Extrinsics**.
- Select the validator's own account (e.g., Charlie) as the sender.
- Choose the `session` pallet and the `setKeys` extrinsic.
- Paste the session keys hex string into the `keys` field.
- Leave the `proof` field as `0x`.
- Submit the transaction.
- Repeat for Dave and Eve.

---

## 3. Register Validators On-Chain
- While still on alice's account (boot) 
- In Polkadot.js Apps, go to **Developer → Sudo**.
- Select the `validatorManager` pallet and the `registerValidators` extrinsic.
- Enter the AccountIds for Charlie, Dave, and Eve as an array, e.g.:
  ```
  [
    5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y,
    5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy,
    5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw
  ]
  ```
- Submit the transaction using the Sudo (Alice) account.

---

## 4. Wait for Session Rotation
- After 1–2 session rotations (typically a few blocks), the new validators will be added to the active set. You can modify the session rotate times if needed.

---

## 5. Verify Validator Status
- In Polkadot.js Apps, go to **Developer → Chain State**.
- Query `session.validators()` to see the current active validator set.
- Query `validatorManager.validatorsToAdd()` to see any pending additions.
- Query `session.nextKeys(accountId)` for each validator to verify their session keys are registered.

---

## 6. Removing Validators
- In Polkadot.js Apps, go to **Developer → Sudo**.
- Select the `validatorManager` pallet and the `removeValidator` extrinsic.
- Enter the AccountId of the validator to remove.
- Submit the transaction using the Sudo (Alice) account.
- After 1–2 session rotations, the validator will be removed from the active set.

---

## 7. Troubleshooting
- **Validator not added?**
  - Ensure the node is running with `--validator` and correct `--bootnodes`.
  - Ensure session keys are set on-chain for the validator's account.
  - Check `session.nextKeys(accountId)` for the validator.
  - Wait for at least one session rotation after setting keys and registering.
- **Validator stuck in `validatorsToAdd()`?**
  - Session keys are likely missing or not set correctly.
- **Database lock or stale state?**
  - Stop all nodes and run:
    ```bash
    rm -rf /tmp/alice /tmp/bob /tmp/charlie /tmp/dave /tmp/eve
    ```
  - Restart your nodes.

---

## 8. Example AccountIds
- Alice:   5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
- Bob:     5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
- Charlie: 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y
- Dave:    5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy
- Eve:     5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw

---

**You can repeat this process to add or remove any validator at runtime.**


Changing the session rotation times:
How to change session rotation time:

Locate the session length parameter in your runtime:

In your runtime code (usually in lib.rs), look for a parameter like:
The name might be SessionPeriod, SessionsPerEra, or similar.
Update the value:

Change the value to your desired number of blocks per session. For example, to rotate every 20 blocks:
Ensure the session pallet uses this parameter:

In your session pallet config, you should see:
If you use PeriodicSessions, the first argument is the session period.
Rebuild your runtime:

Restart your nodes with the new runtime.