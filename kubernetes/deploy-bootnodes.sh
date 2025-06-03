#!/bin/bash

# Deploy Fennel Bootnodes for Public Network
# This script sets up dedicated bootnodes that external validators can connect to

set -e

echo "🌐 Deploying Fennel Public Network Bootnodes..."

# Create namespace for bootnodes if it doesn't exist
kubectl create namespace fennel-bootnodes --dry-run=client -o yaml | kubectl apply -f -

# Create chainspec secret for bootnodes (if not exists)
if ! kubectl get secret fennel-chainspec -n fennel-bootnodes >/dev/null 2>&1; then
    echo "📋 Creating chainspec secret for bootnodes..."
    kubectl create secret generic fennel-chainspec \
        --from-file=fennelSpec.json=./fennelSpec-local.json \
        -n fennel-bootnodes
else
    echo "✅ Chainspec secret already exists"
fi

# Deploy bootnodes using Helm
echo "🚀 Deploying bootnode Helm chart..."
helm upgrade --install fennel-bootnodes \
    oci://ghcr.io/paritytech/helm-charts/node \
    --version v5.1.0 \
    --namespace fennel-bootnodes \
    --values bootnode-values.yaml \
    --wait

echo "⏳ Waiting for bootnodes to be ready..."
kubectl wait --for=condition=ready pod -l app.kubernetes.io/instance=fennel-bootnodes -n fennel-bootnodes --timeout=300s

echo "🔍 Getting bootnode information..."

# Get bootnode node IDs
echo ""
echo "📊 BOOTNODE INFORMATION:"
echo "========================="

for i in 0 1; do
    echo ""
    echo "🔗 Bootnode $i:"
    
    # Get pod name
    POD_NAME="fennel-bootnodes-node-$i"
    
    # Wait for node to start and get node ID from logs
    echo "   Getting node ID..."
    timeout 60 bash -c "
        while true; do
            NODE_ID=\$(kubectl logs -n fennel-bootnodes \$POD_NAME 2>/dev/null | grep -oP 'Local node identity is: \K[a-zA-Z0-9]+' | tail -1)
            if [[ -n \"\$NODE_ID\" ]]; then
                echo \"   Node ID: \$NODE_ID\"
                break
            fi
            sleep 2
        done
    " || echo "   ⚠️  Could not retrieve node ID yet (check logs later)"
    
    # Get service external IP
    SERVICE_NAME="fennel-bootnodes-node-$i-relay-p2p"
    echo "   Getting external IP..."
    kubectl get svc -n fennel-bootnodes $SERVICE_NAME -o wide
done

echo ""
echo "🎯 NEXT STEPS:"
echo "=============="
echo "1. Wait for LoadBalancer external IPs to be assigned"
echo "2. Note down the Node IDs from the logs"
echo "3. Update the public documentation with bootnode addresses"
echo "4. External validators can now connect using these bootnodes"
echo ""

# Create a script to extract bootnode addresses for documentation
cat > get-bootnode-addresses.sh << 'EOF'
#!/bin/bash
echo "🔗 FENNEL NETWORK BOOTNODE ADDRESSES"
echo "====================================="
echo ""

for i in 0 1; do
    echo "Bootnode $i:"
    POD_NAME="fennel-bootnodes-node-$i"
    SERVICE_NAME="fennel-bootnodes-node-$i-relay-p2p"
    
    # Get node ID
    NODE_ID=$(kubectl logs -n fennel-bootnodes $POD_NAME 2>/dev/null | grep -oP 'Local node identity is: \K[a-zA-Z0-9]+' | tail -1)
    
    # Get external IP
    EXTERNAL_IP=$(kubectl get svc -n fennel-bootnodes $SERVICE_NAME -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>/dev/null)
    
    if [[ -n "$NODE_ID" && -n "$EXTERNAL_IP" ]]; then
        echo "  Multiaddr: /ip4/$EXTERNAL_IP/tcp/30333/p2p/$NODE_ID"
        echo "  For Docker: --bootnodes \"/ip4/$EXTERNAL_IP/tcp/30333/p2p/$NODE_ID\""
    else
        echo "  ⚠️  Not ready yet (Node ID: $NODE_ID, IP: $EXTERNAL_IP)"
    fi
    echo ""
done
EOF

chmod +x get-bootnode-addresses.sh

echo "📝 Created 'get-bootnode-addresses.sh' script to get bootnode addresses"
echo "   Run this script once LoadBalancer IPs are assigned"
echo ""
echo "✅ Bootnode deployment complete!" 