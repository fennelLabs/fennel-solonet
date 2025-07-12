#!/bin/bash

# Script to derive public keys from private keys to verify our offline keys are corrupted

echo "=== Deriving public keys from Vault private keys ==="

# Validator 1 keys from Vault
echo "Validator 1:"
echo "Aura seed: 0x721aaee6982d5272bc85a3e8e50e47b5f108fc3a0326a3cc9550d682a28e579d"
echo "Grandpa seed: 0x78608f3e2a3c545960b472de3b9fb82630864d13d7a8e26ab9e897931defe99c"

# Derive Aura public key (sr25519)
aura1_public=$(./target/release/fennel-node key inspect --scheme sr25519 "0x721aaee6982d5272bc85a3e8e50e47b5f108fc3a0326a3cc9550d682a28e579d" | grep "Public key" | cut -d':' -f2 | tr -d ' ')
echo "Derived Aura public: $aura1_public"

# Derive Grandpa public key (ed25519)  
grandpa1_public=$(./target/release/fennel-node key inspect --scheme ed25519 "0x78608f3e2a3c545960b472de3b9fb82630864d13d7a8e26ab9e897931defe99c" | grep "Public key" | cut -d':' -f2 | tr -d ' ')
echo "Derived Grandpa public: $grandpa1_public"

echo ""
echo "Validator 2:"
echo "Aura seed: 0xea9bcfd9d6d7eb3711b38d293490f8f846480a192e247c0619c822e70a523ab0"
echo "Grandpa seed: 0x101cbaf82b3b4f1dde746b26999b29c4044ce15e57de317efc69f7963827d141"

# Derive Aura public key (sr25519)
aura2_public=$(./target/release/fennel-node key inspect --scheme sr25519 "0xea9bcfd9d6d7eb3711b38d293490f8f846480a192e247c0619c822e70a523ab0" | grep "Public key" | cut -d':' -f2 | tr -d ' ')
echo "Derived Aura public: $aura2_public"

# Derive Grandpa public key (ed25519)
grandpa2_public=$(./target/release/fennel-node key inspect --scheme ed25519 "0x101cbaf82b3b4f1dde746b26999b29c4044ce15e57de317efc69f7963827d141" | grep "Public key" | cut -d':' -f2 | tr -d ' ')
echo "Derived Grandpa public: $grandpa2_public"

echo ""
echo "=== Comparison with stored offline keys ==="
echo "Offline Aura public (both validators): 0x46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a"
echo "Offline Grandpa public (both validators): 0x345071da55e5dccefaaa440339415ef9f2663338a38f7da0df21be5ab4e055ef"

echo ""
echo "=== Verification ==="
if [ "$aura1_public" != "0x46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a" ]; then
    echo "❌ Validator 1 Aura public key mismatch - offline keys are corrupted!"
else
    echo "✅ Validator 1 Aura public key matches"
fi

if [ "$aura2_public" != "0x46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a" ]; then
    echo "❌ Validator 2 Aura public key mismatch - offline keys are corrupted!"
else
    echo "✅ Validator 2 Aura public key matches"
fi

if [ "$aura1_public" = "$aura2_public" ]; then
    echo "❌ Both validators have the same Aura public key - this is a critical error!"
else
    echo "✅ Validators have different Aura public keys (correct)"
fi
