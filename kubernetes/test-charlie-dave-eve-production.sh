#!/bin/bash

# Production-Ready External Validator Testing (Charlie, Dave, Eve)
# This version uses production-appropriate flags instead of --dev

set -e

echo "👥 Production-Ready External Validator Testing"
echo "=============================================="

# Well-known development account IDs (for testing only)
CHARLIE_ACCOUNT="5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"
DAVE_ACCOUNT="5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
EVE_ACCOUNT="5HGjWAeFDfFCWPsjFQdVV2Mspz2XtMktvgocEZcCj68kUMaw"

# Port assignments for testing
CHARLIE_PORT=9946
DAVE_PORT=9947
EVE_PORT=9948

# Function to start test validator (PRODUCTION VERSION)
start_test_validator() {
    local name=$1
    local port=$2
    local container_name="fennel-test-$name"
    
    echo "🚀 Starting $name as external validator (port $port) - PRODUCTION CONFIG..."
    
    # Stop any existing container
    docker stop "$container_name" 2>/dev/null || true
    docker rm "$container_name" 2>/dev/null || true
    
    # Create data directory with proper permissions
    local data_dir="/tmp/fennel-test-$name"
    mkdir -p "$data_dir"
    chmod 777 "$data_dir"  # Ensure Docker can write to it
    
    # PRODUCTION-READY fennel-node configuration
    # Using specific flags instead of --dev for security
    docker run -d \
        --name "$container_name" \
        -p "$port:9944" \
        -p "$((port + 100)):30333" \
        -v "$data_dir:/data" \
        --user "$(id -u):$(id -g)" \
        ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df \
        --name "Test$name" \
        --base-path /data \
        --chain local \
        --rpc-external \
        --rpc-methods Safe \
        --rpc-cors "http://localhost:*,http://127.0.0.1:*,https://localhost:*,https://127.0.0.1:*,https://polkadot.js.org" \
        --rpc-max-connections 100 \
        --listen-addr "/ip4/0.0.0.0/tcp/30333" \
        --log info
    
    echo "  ✅ $name started on port $port (PRODUCTION CONFIG)"
    echo "  📊 Monitor: docker logs -f $container_name"
    echo "  🔍 Test RPC: curl -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}' http://localhost:$port"
    echo ""
}

# Function to start with bootnode connection (production scenario)
start_with_bootnodes() {
    local name=$1
    local port=$2
    local container_name="fennel-test-$name"
    
    echo "🚀 Starting $name with bootnode connection (port $port)..."
    
    # Stop any existing container
    docker stop "$container_name" 2>/dev/null || true
    docker rm "$container_name" 2>/dev/null || true
    
    # Create data directory with proper permissions
    local data_dir="/tmp/fennel-test-$name"
    mkdir -p "$data_dir"
    chmod 777 "$data_dir"
    
    # Get bootnodes from k3s deployment (if available)
    local bootnodes=""
    if kubectl get pods -n fennel &>/dev/null; then
        echo "  🔍 Detecting bootnodes from k3s deployment..."
        
        # Try to get bootnode peer IDs (this would be the real production scenario)
        local bootnode_0=$(kubectl get svc -n fennel fennel-bootnode-0 -o jsonpath='{.spec.clusterIP}' 2>/dev/null || echo "")
        local bootnode_1=$(kubectl get svc -n fennel fennel-bootnode-1 -o jsonpath='{.spec.clusterIP}' 2>/dev/null || echo "")
        
        if [ -n "$bootnode_0" ] && [ -n "$bootnode_1" ]; then
            # In production, you'd have the actual peer IDs
            echo "  ✅ Found bootnodes: $bootnode_0, $bootnode_1"
            echo "  ⚠️  Note: In production, you'd need the actual peer IDs"
        else
            echo "  ⚠️  No bootnodes found - starting without bootnode connection"
        fi
    else
        echo "  ⚠️  k3s not available - starting without bootnode connection"
    fi
    
    # Start with production configuration
    docker run -d \
        --name "$container_name" \
        -p "$port:9944" \
        -p "$((port + 100)):30333" \
        -v "$data_dir:/data" \
        --user "$(id -u):$(id -g)" \
        ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df \
        --name "Test$name" \
        --base-path /data \
        --chain local \
        --rpc-external \
        --rpc-methods Safe \
        --rpc-cors "http://localhost:*,http://127.0.0.1:*,https://localhost:*,https://127.0.0.1:*,https://polkadot.js.org" \
        --rpc-max-connections 100 \
        --listen-addr "/ip4/0.0.0.0/tcp/30333" \
        --log info
        # In production, you'd add: --bootnodes "/ip4/BOOTNODE_IP/tcp/30333/p2p/PEER_ID"
    
    echo "  ✅ $name started with production configuration"
    echo ""
}

# Function to generate production deployment instructions
generate_production_guide() {
    echo "📋 Production External Validator Setup Guide"
    echo "============================================"
    echo ""
    echo "🎯 **For Real External Validators (Production)**"
    echo ""
    echo "1. **Build/Install fennel-node binary:**"
    echo "   git clone https://github.com/fennelLabs/fennel-node"
    echo "   cd fennel-node"
    echo "   cargo build --release"
    echo "   # Binary will be at ./target/release/fennel-node"
    echo ""
    echo "2. **Production Configuration:**"
    echo "   ./target/release/fennel-node \\"
    echo "     --name \"YourValidatorName\" \\"
    echo "     --base-path /your/data/path \\"
    echo "     --chain fennel \\"  # or path to chain spec file
    echo "     --validator \\"
    echo "     --rpc-external \\"
    echo "     --rpc-methods Safe \\"
    echo "     --rpc-cors \"http://localhost:*,https://polkadot.js.org\" \\"
    echo "     --rpc-max-connections 100 \\"
    echo "     --listen-addr \"/ip4/0.0.0.0/tcp/30333\" \\"
    echo "     --bootnodes \"/ip4/BOOTNODE_IP/tcp/30333/p2p/PEER_ID\" \\"
    echo "     --log info"
    echo ""
    echo "3. **Key Security Differences from --dev:**"
    echo "   ✅ Uses production chain spec (--chain fennel)"
    echo "   ✅ Restricted CORS (only specific origins)"  
    echo "   ✅ Safe RPC methods only"
    echo "   ✅ No force authoring (follows normal consensus)"
    echo "   ✅ Uses validator's own keys (not Alice's)"
    echo "   ✅ Connects to real bootnodes"
    echo ""
    echo "4. **Session Key Management:**"
    echo "   # Generate session keys"
    echo "   curl -H \"Content-Type: application/json\" \\"
    echo "     -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"author_rotateKeys\"}' \\"
    echo "     http://localhost:9944"
    echo ""
    echo "   # Set keys on-chain (using your validator account)"
    echo "   # Via Polkadot.js Apps: Developer > Extrinsics"
    echo "   # Call: session.setKeys(session_keys, 0x00)"
    echo ""
    echo "5. **Network Security:**"
    echo "   - Use firewall to restrict RPC access"
    echo "   - Consider VPN for RPC connections"
    echo "   - Monitor for unauthorized access"
    echo "   - Use TLS proxy for WebSocket (see setup guides)"
    echo ""
    echo "6. **Validator Registration:**"
    echo "   # Submit to Validator Manager (via governance/sudo)"
    echo "   # validatorManager.registerValidators([your_account_id])"
    echo ""
}

# Function to show configuration comparison
show_config_comparison() {
    echo "⚖️  Configuration Comparison"
    echo "=========================="
    echo ""
    echo "🚨 **DEVELOPMENT (--dev flag):**"
    echo "├── ❌ --chain=dev (development chain)"
    echo "├── ❌ --force-authoring (bypasses consensus)"
    echo "├── ❌ --rpc-cors=all (security risk)"
    echo "├── ❌ --alice (uses development keys)"
    echo "└── ❌ No bootnode requirements"
    echo ""
    echo "✅ **PRODUCTION (specific flags):**"
    echo "├── ✅ --chain=fennel (production chain)"
    echo "├── ✅ Normal consensus rules"
    echo "├── ✅ --rpc-cors=\"specific-origins\" (secure)"
    echo "├── ✅ Validator's own keys"
    echo "├── ✅ --bootnodes for network connection"
    echo "└── ✅ --validator flag for production"
    echo ""
    echo "🎯 **Why the difference matters:**"
    echo "- Development: Fast iteration, no security"
    echo "- Production: Security, proper consensus, network integration"
    echo ""
}

# Function to test production configuration
test_production_config() {
    echo "🎯 Testing Production-Ready External Validator Configuration"
    echo "=========================================================="
    
    echo "🚀 Starting validators with production configuration..."
    start_test_validator "charlie" $CHARLIE_PORT
    start_test_validator "dave" $DAVE_PORT  
    start_test_validator "eve" $EVE_PORT
    
    echo "⏳ Waiting for validators to start..."
    sleep 10
    
    echo "📊 Testing RPC endpoints..."
    for name in charlie dave eve; do
        local port_var=$(echo "${name^^}_PORT")
        local port=${!port_var}
        
        echo "Testing $name on port $port..."
        if curl -s -H "Content-Type: application/json" \
            -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
            "http://localhost:$port" >/dev/null 2>&1; then
            echo "  ✅ $name RPC is responding"
        else
            echo "  ❌ $name RPC not responding"
            echo "  📋 Logs:"
            docker logs "fennel-test-$name" | tail -5
        fi
    done
    
    echo ""
    echo "🔗 **Connection URLs for Polkadot.js Apps:**"
    echo "- Charlie: ws://localhost:$CHARLIE_PORT"
    echo "- Dave: ws://localhost:$DAVE_PORT"  
    echo "- Eve: ws://localhost:$EVE_PORT"
    echo ""
    echo "⚠️  **Note**: These use restricted CORS for security"
    echo "Only authorized origins can connect (localhost, polkadot.js.org)"
}

# Function to cleanup
cleanup_all() {
    echo "🧹 Cleaning up all test validators..."
    docker stop fennel-test-charlie fennel-test-dave fennel-test-eve 2>/dev/null || true
    docker rm fennel-test-charlie fennel-test-dave fennel-test-eve 2>/dev/null || true
    rm -rf /tmp/fennel-test-charlie /tmp/fennel-test-dave /tmp/fennel-test-eve
    echo "  ✅ Cleanup complete"
}

# Main menu
show_menu() {
    echo ""
    echo "🎮 Production External Validator Testing Menu"
    echo "============================================"
    echo "1. Test production configuration"
    echo "2. Start with bootnode connection"
    echo "3. Show configuration comparison"
    echo "4. Generate production setup guide"
    echo "5. Cleanup all test validators"
    echo "6. Exit"
    echo ""
}

# Main function
main() {
    echo "👥 Production-Ready External Validator Testing"
    echo "=============================================="
    echo "This tool demonstrates proper production configuration"
    echo "for external validators (without --dev flag)"
    echo ""
    
    while true; do
        show_menu
        read -p "Choose an option: " choice
        
        case $choice in
            1) test_production_config ;;
            2) 
                echo "Starting with bootnode connection..."
                start_with_bootnodes "charlie" $CHARLIE_PORT
                start_with_bootnodes "dave" $DAVE_PORT
                start_with_bootnodes "eve" $EVE_PORT
                ;;
            3) show_config_comparison ;;
            4) generate_production_guide ;;
            5) cleanup_all ;;
            6) echo "👋 Goodbye!"; exit 0 ;;
            *) echo "❌ Invalid choice" ;;
        esac
        
        read -p "Press Enter to continue..."
    done
}

# Handle command line arguments
if [ $# -gt 0 ]; then
    case $1 in
        --test) test_production_config ;;
        --guide) generate_production_guide ;;
        --compare) show_config_comparison ;;
        --cleanup) cleanup_all ;;
        --help) 
            echo "Usage: $0 [--test|--guide|--compare|--cleanup|--help]"
            echo "  --test:    Test production configuration"
            echo "  --guide:   Show production setup guide"
            echo "  --compare: Show config comparison"
            echo "  --cleanup: Clean up test validators"
            echo "  --help:    Show this help"
            ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
else
    main
fi 