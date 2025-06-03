#!/bin/bash

# Deploy Fennel Bootnodes for Public Network (FIXED VERSION)
# This script sets up dedicated bootnodes that external validators can connect to

set -e

echo "🌐 Deploying Fennel Public Network Bootnodes..."

# Path to local Parity charts
LOCAL_CHART_PATH="/home/neurosx/WORKING_WORKSPACE/Cloud Infrastructure Fennel/GENERIC-ECO-DIRECTORIES/helm-charts/charts/node"

# Verify local chart exists
if [[ ! -d "$LOCAL_CHART_PATH" ]]; then
    echo "❌ Local chart not found at: $LOCAL_CHART_PATH"
    echo "Please ensure you have the Parity helm-charts repository cloned"
    exit 1
fi

echo "✅ Using local chart at: $LOCAL_CHART_PATH"

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

# Deploy bootnodes using local Helm chart
echo "🚀 Deploying bootnode Helm chart..."
helm upgrade --install fennel-bootnodes \
    "$LOCAL_CHART_PATH" \
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
    kubectl get svc -n fennel-bootnodes $SERVICE_NAME -o wide 2>/dev/null || echo "   Service not found yet"
done

echo ""
echo "🎯 NEXT STEPS:"
echo "=============="
echo "1. Wait for LoadBalancer external IPs to be assigned (if using cloud)"
echo "2. Note down the Node IDs from the logs"
echo "3. For local testing, use kubectl port-forward to access bootnodes"
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
    
    # Get external IP (for cloud deployments)
    EXTERNAL_IP=$(kubectl get svc -n fennel-bootnodes $SERVICE_NAME -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>/dev/null)
    
    # Get local cluster IP (for local testing)
    CLUSTER_IP=$(kubectl get svc -n fennel-bootnodes $SERVICE_NAME -o jsonpath='{.spec.clusterIP}' 2>/dev/null)
    
    if [[ -n "$NODE_ID" ]]; then
        if [[ -n "$EXTERNAL_IP" ]]; then
            echo "  External Multiaddr: /ip4/$EXTERNAL_IP/tcp/30333/p2p/$NODE_ID"
            echo "  For Docker: --bootnodes \"/ip4/$EXTERNAL_IP/tcp/30333/p2p/$NODE_ID\""
        elif [[ -n "$CLUSTER_IP" ]]; then
            echo "  Cluster Multiaddr: /ip4/$CLUSTER_IP/tcp/30333/p2p/$NODE_ID"
            echo "  For local testing: kubectl port-forward -n fennel-bootnodes svc/$SERVICE_NAME 30333:30333"
        else
            echo "  ⚠️  Service not ready yet"
        fi
    else
        echo "  ⚠️  Node ID not available yet (check logs: kubectl logs -n fennel-bootnodes $POD_NAME)"
    fi
    echo ""
done
EOF

chmod +x get-bootnode-addresses.sh

echo "📝 Created 'get-bootnode-addresses.sh' script to get bootnode addresses"
echo "   Run this script once services are ready"
echo ""
echo "✅ Bootnode deployment complete!" 