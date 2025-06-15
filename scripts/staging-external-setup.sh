#!/bin/bash

echo "=== Staging Setup for External Validator Testing ==="
echo

echo "Option 1: Update chain_spec.rs with both internal and external addresses:"
echo
cat << 'EOF'
.with_boot_nodes(vec![
    // Internal addresses for nodes inside Kubernetes
    "/dns/fennel-bootnode-1.fennel-staging.svc.cluster.local/tcp/30333/p2p/12D3KooW...".parse().unwrap(),
    "/dns/fennel-bootnode-2.fennel-staging.svc.cluster.local/tcp/30333/p2p/12D3KooW...".parse().unwrap(),
    
    // External addresses for validators outside Kubernetes
    "/ip4/<EXTERNAL_IP_1>/tcp/30333/p2p/12D3KooW...".parse().unwrap(),
    "/ip4/<EXTERNAL_IP_2>/tcp/30333/p2p/12D3KooW...".parse().unwrap(),
])
EOF

echo
echo "Option 2: Use a separate chain spec for external validators:"
echo
echo "1. Create staging-external.json with only external bootnode addresses"
echo "2. Distribute this to external validators"
echo "3. They run: ./fennel-node --chain staging-external.json"

echo
echo "Steps to expose bootnodes:"
echo
echo "1. Apply the LoadBalancer service:"
echo "   kubectl apply -f k8s/staging-bootnode-external.yaml"
echo
echo "2. Get external IPs:"
echo "   kubectl get svc -n fennel-staging"
echo
echo "3. Update chain_spec.rs with the external IPs"
echo
echo "4. Rebuild and generate new chain specs"

echo
echo "For external validators to join:"
echo
cat << 'EOF'
# External validator command:
./fennel-node \
  --chain staging_raw.json \
  --validator \
  --name "External-Validator-1" \
  --port 30333 \
  --rpc-port 9944 \
  --base-path /tmp/validator1
EOF

echo
echo "Security considerations for staging:"
echo "- Use --reserved-only and --reserved-nodes for controlled access"
echo "- Implement firewall rules to limit access"
echo "- Monitor for unexpected connections"
echo "- This is for TESTING only - production needs proper security" 