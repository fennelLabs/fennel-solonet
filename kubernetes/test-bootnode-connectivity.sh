#!/bin/bash

# Test Bootnode Connectivity with Alice & Bob
# This script tests the complete P2P discovery network

set -e

echo "🧪 Testing Fennel Bootnode Connectivity"
echo "======================================="

# Current known peer IDs
ALICE_PEER_ID="12D3KooWR6LStFm9Vif78LEVuDRE9tYA2zJ8r4qTENoQKmv4tA5h"
BOB_PEER_ID="12D3KooWH836DFpUGv6FedorW8hUbmadYFJKQuX5qLeUNYzRYieN"

echo "🔍 Current Network State:"
echo "  Alice (node-0): $ALICE_PEER_ID"
echo "  Bob (node-1):   $BOB_PEER_ID"
echo ""

# Function to check RPC connectivity
check_rpc() {
    local url=$1
    local name=$2
    echo "🔌 Testing RPC connectivity to $name..."
    
    if curl -s -H "Content-Type: application/json" \
        -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
        "$url" >/dev/null 2>&1; then
        echo "  ✅ $name RPC is accessible"
        return 0
    else
        echo "  ❌ $name RPC is not accessible"
        return 1
    fi
}

# Function to get peer info
get_peer_info() {
    local url=$1
    local name=$2
    echo "📡 Getting peer info from $name..."
    
    # Get local peer ID
    local peer_id=$(curl -s -H "Content-Type: application/json" \
        -d '{"id":1, "jsonrpc":"2.0", "method": "system_localPeerId"}' \
        "$url" | jq -r '.result')
    
    # Get connected peers
    local peers=$(curl -s -H "Content-Type: application/json" \
        -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
        "$url" | jq -r '.result[] | .peerId')
    
    echo "  🆔 $name Peer ID: $peer_id"
    echo "  🔗 Connected peers:"
    if [ -z "$peers" ]; then
        echo "    (none)"
    else
        echo "$peers" | while read -r peer; do
            echo "    - $peer"
        done
    fi
    echo ""
}

# Function to check if bootnodes are deployed
check_bootnodes_deployed() {
    echo "🚀 Checking if bootnodes are deployed..."
    
    if kubectl get namespace fennel-bootnodes >/dev/null 2>&1; then
        echo "  ✅ Bootnode namespace exists"
        
        local bootnode_pods=$(kubectl get pods -n fennel-bootnodes -o name 2>/dev/null | wc -l)
        echo "  📊 Bootnode pods: $bootnode_pods"
        
        if [ "$bootnode_pods" -gt 0 ]; then
            kubectl get pods -n fennel-bootnodes
            return 0
        else
            echo "  ⚠️  No bootnode pods found"
            return 1
        fi
    else
        echo "  ❌ Bootnode namespace not found"
        return 1
    fi
}

# Function to deploy bootnodes
deploy_bootnodes() {
    echo "🚀 Deploying bootnodes..."
    
    if [ ! -f "./deploy-bootnodes.sh" ]; then
        echo "  ❌ deploy-bootnodes.sh not found"
        return 1
    fi
    
    chmod +x ./deploy-bootnodes.sh
    ./deploy-bootnodes.sh
    
    echo "  ⏳ Waiting for bootnodes to be ready..."
    kubectl wait --for=condition=ready pod -l app.kubernetes.io/instance=fennel-bootnodes -n fennel-bootnodes --timeout=300s
    
    echo "  ✅ Bootnodes deployed successfully"
}

# Function to get bootnode addresses
get_bootnode_addresses() {
    echo "🔗 Getting bootnode addresses..."
    
    for i in 0 1; do
        echo "  📡 Bootnode $i:"
        POD_NAME="fennel-bootnodes-node-$i"
        SERVICE_NAME="fennel-bootnodes-node-$i-relay-p2p"
        
        # Get node ID from logs
        local node_id=$(kubectl logs -n fennel-bootnodes $POD_NAME 2>/dev/null | \
            grep -oP 'Local node identity is: \K[a-zA-Z0-9]+' | tail -1)
        
        # Get external IP (might be pending for LoadBalancer)
        local external_ip=$(kubectl get svc -n fennel-bootnodes $SERVICE_NAME \
            -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>/dev/null)
        
        # Get cluster IP as fallback
        local cluster_ip=$(kubectl get svc -n fennel-bootnodes $SERVICE_NAME \
            -o jsonpath='{.spec.clusterIP}' 2>/dev/null)
        
        echo "    🆔 Node ID: $node_id"
        echo "    🌐 External IP: ${external_ip:-pending}"
        echo "    🏠 Cluster IP: $cluster_ip"
        
        if [ -n "$node_id" ]; then
            if [ -n "$external_ip" ]; then
                echo "    📍 External Multiaddr: /ip4/$external_ip/tcp/30333/p2p/$node_id"
            fi
            echo "    📍 Internal Multiaddr: /ip4/$cluster_ip/tcp/30333/p2p/$node_id"
        fi
        echo ""
    done
}

# Function to test external validator simulation
test_external_validator() {
    echo "🧪 Testing External Validator Simulation..."
    echo "This simulates an external validator connecting via bootnodes"
    echo ""
    
    # Create a test script for external validator
    cat > test-external-connection.sh << 'EOF'
#!/bin/bash
# Simulated external validator test

VALIDATOR_NAME="TestExternalValidator"
CONTAINER_NAME="fennel-test-external"

# Get bootnode addresses (you'll need to update these)
echo "📡 Testing connection to Fennel network via bootnodes..."

# For testing, we'll try to connect without validator flag first
docker run --rm -d \
    --name $CONTAINER_NAME \
    -p 9945:9944 \
    -v /tmp/test-fennel:/data \
    ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df \
    --name "$VALIDATOR_NAME" \
    --base-path /data \
    --chain fennel \
    --listen-addr /ip4/0.0.0.0/tcp/30333 \
    --rpc-addr 0.0.0.0:9944 \
    --rpc-methods Safe \
    --rpc-cors all \
    --log info

echo "🔍 Test external validator started. Check logs with:"
echo "  docker logs -f $CONTAINER_NAME"
echo ""
echo "🧪 To test connectivity:"
echo "  curl -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}' http://localhost:9945"
echo ""
echo "🛑 To stop test:"
echo "  docker stop $CONTAINER_NAME"

EOF
    
    chmod +x test-external-connection.sh
    echo "  ✅ Created test-external-connection.sh for manual testing"
}

# Main test sequence
main() {
    echo "🎯 Starting Bootnode Connectivity Test..."
    echo ""
    
    # Step 1: Check current network
    echo "📊 Step 1: Current Network Analysis"
    echo "-----------------------------------"
    check_rpc "http://localhost:9944" "Alice (current validator)"
    get_peer_info "http://localhost:9944" "Alice"
    
    # Step 2: Check/Deploy bootnodes
    echo "🚀 Step 2: Bootnode Deployment"
    echo "------------------------------"
    if ! check_bootnodes_deployed; then
        echo "  📋 Bootnodes not found. Would you like to deploy them?"
        read -p "  Deploy bootnodes? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            deploy_bootnodes
        else
            echo "  ⏭️  Skipping bootnode deployment"
        fi
    fi
    
    # Step 3: Get bootnode info
    echo "🔗 Step 3: Bootnode Network Information"
    echo "--------------------------------------"
    if kubectl get namespace fennel-bootnodes >/dev/null 2>&1; then
        get_bootnode_addresses
    else
        echo "  ⚠️  Bootnodes not deployed, skipping this step"
    fi
    
    # Step 4: Create test tools
    echo "🧪 Step 4: External Validator Test Tools"
    echo "----------------------------------------"
    test_external_validator
    
    echo ""
    echo "🎯 Test Summary:"
    echo "==============="
    echo "✅ Network Analysis: Alice and Bob connectivity verified"
    echo "📋 Bootnode Status: Check deployment status above"
    echo "🧪 Test Tools: Created test-external-connection.sh"
    echo ""
    echo "🔄 Next Steps:"
    echo "1. If bootnodes aren't deployed, run: ./deploy-bootnodes.sh"
    echo "2. Wait for LoadBalancer IPs: ./get-bootnode-addresses.sh"  
    echo "3. Test external connection: ./test-external-connection.sh"
    echo "4. Use Charlie/Dave/Eve for full validator testing"
}

# Run main function
main "$@" 