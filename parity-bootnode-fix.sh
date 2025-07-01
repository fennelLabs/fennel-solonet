#!/bin/bash

# Fennel Bootnode Connectivity Fix Script
# Based on official Parity documentation and best practices
# References:
# - https://docs.polkadot.com/infrastructure/running-a-node/setup-bootnode/
# - https://paritytech.github.io/devops-guide/guides/readiness-checklist.html

set -euo pipefail

echo "🔧 Fennel Bootnode Connectivity Fix (Parity Best Practices)"
echo "============================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Function to test bootnode using Parity's recommended method
test_bootnode_parity_method() {
    local bootnode_addr=$1
    local test_name=$2
    
    print_status "Testing $test_name bootnode connectivity using Parity method..."
    echo "Bootnode address: $bootnode_addr"
    
    # Create a temporary test directory
    local test_dir="/tmp/bootnode-test-$(date +%s)"
    mkdir -p "$test_dir"
    
    # Test using the method from Parity docs
    timeout 30 fennel-node \
        --chain chainspecs/staging/staging-chainspec.json \
        --base-path "$test_dir" \
        --name "Bootnode-Test-Node" \
        --reserved-only \
        --reserved-nodes "$bootnode_addr" \
        --no-hardware-benchmarks \
        --tmp 2>&1 | grep -q "syncing.*peers" && {
        print_success "$test_name bootnode is accessible!"
        cleanup_test_dir "$test_dir"
        return 0
    } || {
        print_error "$test_name bootnode is NOT accessible"
        cleanup_test_dir "$test_dir"
        return 1
    }
}

cleanup_test_dir() {
    local dir=$1
    rm -rf "$dir" 2>/dev/null || true
}

# Function to extract peer ID from running bootnode
get_bootnode_peer_id() {
    print_status "Extracting actual peer ID from running bootnode..."
    
    # Get the actual peer ID from the running bootnode
    local peer_id=$(kubectl logs -n fennel-staging bootnode-0 --tail=100 2>/dev/null | grep -o "Local node identity is:.*" | head -1 | grep -o "12D3[A-Za-z0-9]*")
    
    if [ -n "$peer_id" ]; then
        print_success "Found bootnode peer ID: $peer_id"
        echo "$peer_id"
    else
        print_warning "Could not extract peer ID from bootnode logs"
        # Fallback to the one we see in the running processes
        echo "12D3KooWRpzRTivvJ5ySvgbFnPeEE6rDhitQKL1fFJvvBGhnenSk"
    fi
}

# Function to create working bootnode addresses
create_working_bootnode_addresses() {
    print_status "Creating working bootnode addresses based on current cluster setup..."
    
    local peer_id=$(get_bootnode_peer_id)
    local cluster_ip=$(kubectl get svc -n fennel-staging bootnode-0-relay-chain-p2p -o jsonpath='{.spec.clusterIP}')
    local node_port=$(kubectl get svc -n fennel-staging bootnode-0-relay-chain-p2p -o jsonpath='{.spec.ports[0].nodePort}')
    
    # Create array of working bootnode addresses
    local working_bootnodes=(
        # Internal cluster DNS
        "/dns4/bootnode-0-relay-chain-p2p.fennel-staging.svc.cluster.local/tcp/30333/p2p/${peer_id}"
        # Cluster IP (internal)
        "/ip4/${cluster_ip}/tcp/30333/p2p/${peer_id}"
    )
    
    # If NodePort is available, add external access option
    if [ -n "$node_port" ] && [ "$node_port" != "null" ]; then
        working_bootnodes+=("/ip4/127.0.0.1/tcp/${node_port}/p2p/${peer_id}")
    fi
    
    printf '%s\n' "${working_bootnodes[@]}"
}

# Function to update chain specs with working bootnodes
update_chainspecs_with_working_bootnodes() {
    print_status "Updating chain specifications with working bootnodes..."
    
    local chainspecs=("chainspecs/staging/staging-chainspec.json" "chainspecs/development/development.json")
    
    # Get working bootnode addresses
    local working_bootnodes
    mapfile -t working_bootnodes < <(create_working_bootnode_addresses)
    
    # Create JSON array
    local bootnode_json
    bootnode_json=$(printf '%s\n' "${working_bootnodes[@]}" | jq -R . | jq -s .)
    
    for chainspec in "${chainspecs[@]}"; do
        if [ -f "$chainspec" ]; then
            # Backup original
            cp "$chainspec" "${chainspec}.backup-$(date +%s)"
            
            # Update bootNodes field
            jq --argjson bootnodes "$bootnode_json" '.bootNodes = $bootnodes' "$chainspec" > "${chainspec}.tmp"
            mv "${chainspec}.tmp" "$chainspec"
            
            print_success "Updated $chainspec with working bootnodes"
            
            # Show what was added
            echo "New bootNodes:"
            jq '.bootNodes[]' "$chainspec" | sed 's/^/  /'
        else
            print_warning "Chain spec not found: $chainspec"
        fi
    done
}

# Function to diagnose using Parity's debug methods
diagnose_with_parity_debug() {
    print_status "Running Parity-style diagnostics..."
    
    echo ""
    echo "=== Node Peer Status ==="
    for node in fennel-validators-node-0 fennel-validators-node-1 fennel-validators-node-2 bootnode-0; do
        local peer_count=$(kubectl logs -n fennel-staging "$node" --tail=10 2>/dev/null | grep -o "([0-9]\+ peers)" | tail -1 | grep -o "[0-9]\+" || echo "unknown")
        echo "  $node: $peer_count peers"
    done
    
    echo ""
    echo "=== Bootnode Node Identity ==="
    kubectl logs -n fennel-staging bootnode-0 --tail=50 2>/dev/null | grep -E "Local node identity|Started|listen" || echo "  No identity logs found"
    
    echo ""
    echo "=== Network Configuration ==="
    echo "  Kubernetes Services:"
    kubectl get svc -n fennel-staging | grep -E "(bootnode|relay-chain-p2p)" | sed 's/^/    /'
}

# Main execution
print_status "Starting Parity-compliant bootnode diagnostics..."

# Run diagnostics
diagnose_with_parity_debug

# Handle command line arguments
case "${1:-}" in
    --fix)
        echo ""
        print_status "Applying Parity-recommended fixes..."
        update_chainspecs_with_working_bootnodes
        echo ""
        echo "🚀 NEXT STEPS:"
        echo "1. Deploy updated chainspecs to cluster"
        echo "2. Restart nodes with rolling update"
        echo "3. Monitor connectivity logs"
        ;;
    --help)
        echo ""
        echo "Usage: $0 [OPTION]"
        echo ""
        echo "Options:"
        echo "  --fix           Apply Parity-recommended fixes to chainspecs"
        echo "  --help          Show this help message"
        echo ""
        echo "Based on official Parity documentation"
        ;;
    *)
        echo ""
        print_status "Diagnosis complete. Use --fix to apply Parity-recommended solutions."
        ;;
esac

echo ""
print_success "Parity-compliant bootnode analysis complete!" 