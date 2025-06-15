#!/bin/bash

# Quick bootnode setup for Fennel staging network

echo "=== Quick Bootnode Setup for Fennel Staging ==="
echo

# Method 1: Using subkey tool (RECOMMENDED)
echo "Method 1: Using subkey tool (RECOMMENDED):"
echo "Install subkey if not already installed:"
echo "  cargo install --force subkey --git https://github.com/paritytech/polkadot-sdk --tag stable2503"
echo
echo "Generate keys and get peer IDs directly:"
echo "  subkey generate-node-key > bootnode1.key 2> bootnode1_peer_id.txt"
echo "  subkey generate-node-key > bootnode2.key 2> bootnode2_peer_id.txt"
echo
echo "The peer IDs are automatically saved to bootnode1_peer_id.txt and bootnode2_peer_id.txt"
echo "No need to start the nodes to get peer IDs!"
echo

# Method 2: Generate example keys using openssl (fallback method)
echo "Method 2: Using openssl (fallback - requires starting nodes to get peer IDs):"
echo "Generating example node keys..."

# Generate 32-byte hex keys
BOOTNODE1_KEY=$(openssl rand -hex 32)
BOOTNODE2_KEY=$(openssl rand -hex 32)

echo "Bootnode 1 Key: $BOOTNODE1_KEY"
echo "Bootnode 2 Key: $BOOTNODE2_KEY"

# Save keys to files
echo -n "$BOOTNODE1_KEY" > bootnode1-openssl.key
echo -n "$BOOTNODE2_KEY" > bootnode2-openssl.key

echo
echo "Keys saved to bootnode1-openssl.key and bootnode2-openssl.key"
echo "Note: With openssl method, you need to start nodes to get peer IDs"
echo

# Check if peer ID files exist from subkey method
if [ -f "bootnode1_peer_id.txt" ] && [ -f "bootnode2_peer_id.txt" ]; then
    PEER_ID_1=$(cat bootnode1_peer_id.txt)
    PEER_ID_2=$(cat bootnode2_peer_id.txt)
    echo "✅ Found peer IDs from subkey:"
    echo "   Bootnode 1: $PEER_ID_1"
    echo "   Bootnode 2: $PEER_ID_2"
    echo
fi

# Show example multiaddresses
echo "Example multiaddresses for chain_spec.rs:"
echo
if [ -n "$PEER_ID_1" ] && [ -n "$PEER_ID_2" ]; then
    echo "Using your actual peer IDs:"
    cat << EOF
// For local testing:
.with_boot_nodes(vec![
    "/ip4/127.0.0.1/tcp/30333/p2p/$PEER_ID_1".parse().unwrap(),
    "/ip4/127.0.0.1/tcp/30334/p2p/$PEER_ID_2".parse().unwrap(),
])

// For Kubernetes STAGING with BOTH internal and external access:
.with_boot_nodes(vec![
    // Internal addresses for nodes inside Kubernetes
    "/dns/fennel-bootnode-1.fennel-staging.svc.cluster.local/tcp/30333/p2p/$PEER_ID_1".parse().unwrap(),
    "/dns/fennel-bootnode-2.fennel-staging.svc.cluster.local/tcp/30333/p2p/$PEER_ID_2".parse().unwrap(),
    
    // External addresses for validators outside Kubernetes (replace with actual LoadBalancer IPs)
    "/ip4/<EXTERNAL_IP_1>/tcp/30333/p2p/$PEER_ID_1".parse().unwrap(),
    "/ip4/<EXTERNAL_IP_2>/tcp/30333/p2p/$PEER_ID_2".parse().unwrap(),
])
EOF
else
    echo "No peer IDs found. Run subkey generate-node-key first, or use placeholder IDs:"
    cat << 'EOF'
// For local testing (replace with actual peer IDs):
.with_boot_nodes(vec![
    "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp".parse().unwrap(),
    "/ip4/127.0.0.1/tcp/30334/p2p/12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD".parse().unwrap(),
])

// For Kubernetes STAGING with BOTH internal and external access:
.with_boot_nodes(vec![
    // Internal addresses for nodes inside Kubernetes
    "/dns/fennel-bootnode-1.fennel-staging.svc.cluster.local/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp".parse().unwrap(),
    "/dns/fennel-bootnode-2.fennel-staging.svc.cluster.local/tcp/30333/p2p/12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD".parse().unwrap(),
    
    // External addresses for validators outside Kubernetes (replace with actual LoadBalancer IPs)
    "/ip4/<EXTERNAL_IP_1>/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp".parse().unwrap(),
    "/ip4/<EXTERNAL_IP_2>/tcp/30333/p2p/12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD".parse().unwrap(),
])
EOF
fi

echo
echo "=== Simplified Process with Subkey ==="
echo
echo "1. Generate keys and peer IDs (if not already done):"
echo "   subkey generate-node-key > bootnode1.key 2> bootnode1_peer_id.txt"
echo "   subkey generate-node-key > bootnode2.key 2> bootnode2_peer_id.txt"
echo
echo "2. Update chain_spec.rs with the peer IDs from the .txt files"
echo
echo "3. Build your node:"
echo "   cd .. && cargo build --release"
echo
echo "4. Generate chain specs:"
echo "   ./target/release/fennel-node build-spec --chain staging > staging.json"
echo "   ./target/release/fennel-node build-spec --chain staging --raw > staging_raw.json"
echo
echo "5. Deploy to Kubernetes with the keys mounted as secrets"

echo
echo "=== Kubernetes Setup for External Validator Testing ==="
echo
echo "1. Create LoadBalancer services for external access:"
cat << 'EOF'

# Save this as k8s/staging-bootnode-external.yaml
apiVersion: v1
kind: Service
metadata:
  name: fennel-bootnode-1-external
  namespace: fennel-staging
spec:
  type: LoadBalancer
  ports:
    - name: p2p
      port: 30333
      targetPort: 30333
      protocol: TCP
  selector:
    app: fennel-bootnode-1
---
apiVersion: v1
kind: Service
metadata:
  name: fennel-bootnode-2-external
  namespace: fennel-staging
spec:
  type: LoadBalancer
  ports:
    - name: p2p
      port: 30333
      targetPort: 30333
      protocol: TCP
  selector:
    app: fennel-bootnode-2
EOF

echo
echo "2. Apply the external services:"
echo "   kubectl apply -f k8s/staging-bootnode-external.yaml"
echo
echo "3. Get the external IPs (wait for LoadBalancer to provision):"
echo "   kubectl get svc -n fennel-staging | grep external"
echo
echo "4. Update chain_spec.rs with the actual external IPs"
echo
echo "5. Rebuild and regenerate chain specs"
echo

echo "=== External Validator Testing ==="
echo
echo "External validators can join using:"
echo "   ./fennel-node --chain staging_raw.json --validator --name 'External-Val-1'"
echo
echo "To test validator joining/leaving with your validator-manager pallet:"
echo "1. External validator generates session keys:"
echo "   curl -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":2.0, \"method\": \"author_rotateKeys\"}' http://localhost:9944"
echo
echo "2. Register with validator manager (via sudo/Alice):"
echo "   validatorManager.addValidator(externalValidatorAccountId)"
echo
echo "3. To remove validator:"
echo "   validatorManager.removeValidator(externalValidatorAccountId)"
echo
echo "Security note: This exposes your staging network. Consider:"
echo "- Firewall rules to limit access"
echo "- Using --reserved-nodes for controlled testing"
echo "- Monitoring unexpected connections"