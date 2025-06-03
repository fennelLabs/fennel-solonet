#!/bin/bash

# Test Charlie, Dave, and Eve as External Validators
# This simulates the complete external validator onboarding process

set -e

echo "👥 Testing Charlie, Dave & Eve as External Validators"
echo "====================================================="

# Well-known development account IDs
CHARLIE_ACCOUNT="5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"
DAVE_ACCOUNT="5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
EVE_ACCOUNT="5HGjWAeFDfFCWPsjFQdVV2Mspz2XtMktvgocEZcCj68kUMaw"

# Charlie's well-known keys
CHARLIE_AURA_SEED="0x868020ae0687dda7d57565093a69090211449845a7e11453612800b663307246"
CHARLIE_GRANDPA_SEED="0x786ad0e2df456fe43dd1f91ebca22e235bc162e0bb8d53c633e8c85b2af68b7a"

# Dave's well-known keys  
DAVE_AURA_SEED="0x785198d1dd0b95d2736d1000f72e33a4b4bc8f35a3e8b6b9f66c4ba949b8b8b4"
DAVE_GRANDPA_SEED="0x42438b7883391c05512a938e36c2df0131e088b3756d6aa7a755fbff19d2f842"

# Eve's well-known keys
EVE_AURA_SEED="0x15e69915b3e85df8a0e2a2a8e5e8e3d3f8f8e3d3f8f8e3d3f8f8e3d3f8f8e3d3"
EVE_GRANDPA_SEED="0x7e41d57d402bd2c8a5d29b0b8d3e9d8e3d3f8f8e3d3f8f8e3d3f8f8e3d3f8f8"

# Port assignments for testing
CHARLIE_PORT=9946
DAVE_PORT=9947
EVE_PORT=9948

# Function to start test validator
start_test_validator() {
    local name=$1
    local port=$2
    local container_name="fennel-test-$name"
    
    echo "🚀 Starting $name as external validator (port $port)..."
    
    # Stop any existing container
    docker stop "$container_name" 2>/dev/null || true
    docker rm "$container_name" 2>/dev/null || true
    
    # Create data directory
    mkdir -p "/tmp/fennel-test-$name"
    
    # For now, start without bootnodes to test local startup
    # You'll need to update --bootnodes once they're deployed
    docker run -d \
        --name "$container_name" \
        -p "$port:9944" \
        -p "$((port + 100)):30333" \
        -v "/tmp/fennel-test-$name:/data" \
        ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df \
        --name "Test$name" \
        --base-path /data \
        --chain fennel \
        --listen-addr "/ip4/0.0.0.0/tcp/30333" \
        --rpc-addr "0.0.0.0:9944" \
        --rpc-methods Safe \
        --rpc-cors all \
        --rpc-max-connections 100 \
        --log info \
        --no-telemetry
    
    echo "  ✅ $name started on port $port"
    echo "  📊 Monitor: docker logs -f $container_name"
    echo "  🔍 Test RPC: curl -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}' http://localhost:$port"
    echo ""
}

# Function to wait for node to be ready
wait_for_node() {
    local port=$1
    local name=$2
    local timeout=60
    
    echo "⏳ Waiting for $name to be ready..."
    
    for i in $(seq 1 $timeout); do
        if curl -s -H "Content-Type: application/json" \
            -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
            "http://localhost:$port" >/dev/null 2>&1; then
            echo "  ✅ $name is ready!"
            return 0
        fi
        
        if [ $((i % 10)) -eq 0 ]; then
            echo "  ⏳ Still waiting for $name... ($i/$timeout)"
        fi
        
        sleep 1
    done
    
    echo "  ❌ $name failed to start within $timeout seconds"
    return 1
}

# Function to generate session keys
generate_session_keys() {
    local port=$1
    local name=$2
    
    echo "🔑 Generating session keys for $name..."
    
    local session_keys=$(curl -s -H "Content-Type: application/json" \
        -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
        "http://localhost:$port" | jq -r '.result')
    
    if [ "$session_keys" != "null" ] && [ -n "$session_keys" ]; then
        echo "  ✅ $name session keys: $session_keys"
        
        # Save to file for later use
        echo "$session_keys" > "/tmp/fennel-test-$name-session-keys.txt"
        echo "  💾 Saved to: /tmp/fennel-test-$name-session-keys.txt"
        
        # Verify keys are stored
        local has_keys=$(curl -s -H "Content-Type: application/json" \
            -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"author_hasSessionKeys\", \"params\": [\"$session_keys\"]}" \
            "http://localhost:$port" | jq -r '.result')
        
        if [ "$has_keys" = "true" ]; then
            echo "  ✅ Session keys verified in keystore"
        else
            echo "  ⚠️  Session keys not found in keystore"
        fi
    else
        echo "  ❌ Failed to generate session keys for $name"
        return 1
    fi
    echo ""
}

# Function to get node peer ID
get_peer_id() {
    local port=$1
    local name=$2
    
    local peer_id=$(curl -s -H "Content-Type: application/json" \
        -d '{"id":1, "jsonrpc":"2.0", "method": "system_localPeerId"}' \
        "http://localhost:$port" | jq -r '.result')
    
    echo "  🆔 $name Peer ID: $peer_id"
    echo "$peer_id" > "/tmp/fennel-test-$name-peer-id.txt"
}

# Function to check node status
check_node_status() {
    local port=$1
    local name=$2
    
    echo "📊 Checking $name status..."
    
    # Health check
    local health=$(curl -s -H "Content-Type: application/json" \
        -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
        "http://localhost:$port")
    
    echo "  🏥 Health: $(echo "$health" | jq -c '.')"
    
    # Get peer ID
    get_peer_id "$port" "$name"
    
    # Get connected peers
    local peers=$(curl -s -H "Content-Type: application/json" \
        -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
        "http://localhost:$port" | jq -r '.result[] | .peerId')
    
    echo "  🔗 Connected peers:"
    if [ -z "$peers" ]; then
        echo "    (none - this is expected without bootnodes)"
    else
        echo "$peers" | while read -r peer; do
            echo "    - $peer"
        done
    fi
    echo ""
}

# Function to stop test validator
stop_test_validator() {
    local name=$1
    local container_name="fennel-test-$name"
    
    echo "🛑 Stopping $name..."
    docker stop "$container_name" 2>/dev/null || true
    docker rm "$container_name" 2>/dev/null || true
    echo "  ✅ $name stopped"
}

# Function to cleanup all test validators
cleanup_all() {
    echo "🧹 Cleaning up all test validators..."
    stop_test_validator "charlie"
    stop_test_validator "dave" 
    stop_test_validator "eve"
    
    # Clean up data directories
    rm -rf /tmp/fennel-test-charlie /tmp/fennel-test-dave /tmp/fennel-test-eve
    rm -f /tmp/fennel-test-*-session-keys.txt /tmp/fennel-test-*-peer-id.txt
    
    echo "  ✅ Cleanup complete"
}

# Function to create validator registration summary
create_registration_summary() {
    echo "📋 External Validator Registration Summary"
    echo "=========================================="
    echo ""
    
    for validator in charlie dave eve; do
        local name=$(echo "$validator" | sed 's/./\U&/')
        local port_var="${name}_PORT"
        local account_var="${name}_ACCOUNT"
        local port=${!port_var}
        local account=${!account_var}
        
        echo "👤 $name:"
        echo "  💳 Account ID: $account"
        
        if [ -f "/tmp/fennel-test-$validator-peer-id.txt" ]; then
            local peer_id=$(cat "/tmp/fennel-test-$validator-peer-id.txt")
            echo "  🆔 Peer ID: $peer_id"
        fi
        
        if [ -f "/tmp/fennel-test-$validator-session-keys.txt" ]; then
            local session_keys=$(cat "/tmp/fennel-test-$validator-session-keys.txt")
            echo "  🔑 Session Keys: $session_keys"
        fi
        
        echo "  🌐 RPC Port: $port"
        echo "  📊 Status: curl -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}' http://localhost:$port"
        echo ""
    done
    
    echo "🔄 Next Steps for Testing ValidatorManager:"
    echo "1. Connect to Alice via: kubectl port-forward -n fennel fennel-solochain-node-0 9944:9944"
    echo "2. Open Polkadot.js Apps and connect to ws://localhost:9944"
    echo "3. For each validator above:"
    echo "   a. Go to Developer > Extrinsics"
    echo "   b. Select the validator's account as sender"
    echo "   c. Call session.setKeys(session_keys, 0x00)"
    echo "   d. Submit transaction"
    echo "4. Then use Sudo to add them:"
    echo "   a. Go to Developer > Sudo"
    echo "   b. Call validatorManager.registerValidators([account_id])"
    echo "   c. Submit with sudo account"
    echo "5. Monitor session rotation to see them become active validators"
}

# Function to test full workflow
test_full_workflow() {
    echo "🎯 Testing Full External Validator Workflow"
    echo "==========================================="
    
    # Step 1: Start all test validators
    echo "🚀 Step 1: Starting Test Validators"
    echo "-----------------------------------"
    start_test_validator "charlie" $CHARLIE_PORT
    start_test_validator "dave" $DAVE_PORT
    start_test_validator "eve" $EVE_PORT
    
    # Step 2: Wait for all to be ready
    echo "⏳ Step 2: Waiting for Validators to Start"
    echo "------------------------------------------"
    wait_for_node $CHARLIE_PORT "Charlie"
    wait_for_node $DAVE_PORT "Dave"
    wait_for_node $EVE_PORT "Eve"
    
    # Step 3: Generate session keys for all
    echo "🔑 Step 3: Generating Session Keys"
    echo "----------------------------------"
    generate_session_keys $CHARLIE_PORT "charlie"
    generate_session_keys $DAVE_PORT "dave"
    generate_session_keys $EVE_PORT "eve"
    
    # Step 4: Check status of all
    echo "📊 Step 4: Checking Node Status"
    echo "-------------------------------"
    check_node_status $CHARLIE_PORT "Charlie"
    check_node_status $DAVE_PORT "Dave"
    check_node_status $EVE_PORT "Eve"
    
    # Step 5: Create summary
    echo "📋 Step 5: Registration Summary"
    echo "-------------------------------"
    create_registration_summary
}

# Main menu
show_menu() {
    echo ""
    echo "🎮 External Validator Testing Menu"
    echo "=================================="
    echo "1. Test full workflow (Charlie + Dave + Eve)"
    echo "2. Start individual validator"
    echo "3. Check validator status"
    echo "4. Generate session keys"
    echo "5. Show registration summary"
    echo "6. Cleanup all test validators"
    echo "7. Exit"
    echo ""
}

# Handle individual validator actions
handle_individual() {
    echo "Select validator:"
    echo "1. Charlie (port $CHARLIE_PORT)"
    echo "2. Dave (port $DAVE_PORT)"
    echo "3. Eve (port $EVE_PORT)"
    read -p "Choice: " choice
    
    case $choice in
        1) name="charlie"; port=$CHARLIE_PORT ;;
        2) name="dave"; port=$DAVE_PORT ;;
        3) name="eve"; port=$EVE_PORT ;;
        *) echo "Invalid choice"; return ;;
    esac
    
    echo "Selected: $name (port $port)"
    read -p "Action? (start/stop/status/keys): " action
    
    case $action in
        start) start_test_validator "$name" "$port" ;;
        stop) stop_test_validator "$name" ;;
        status) check_node_status "$port" "$name" ;;
        keys) generate_session_keys "$port" "$name" ;;
        *) echo "Invalid action" ;;
    esac
}

# Main interactive loop
main() {
    echo "👥 External Validator Testing Tool"
    echo "=================================="
    echo "This tool helps test Charlie, Dave & Eve as external validators"
    echo ""
    
    # Check if Docker is available
    if ! command -v docker &> /dev/null; then
        echo "❌ Docker is not available. Please install Docker first."
        exit 1
    fi
    
    # Check if image exists
    if ! docker image inspect ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df &> /dev/null; then
        echo "⚠️  Fennel Docker image not found locally. Pulling..."
        docker pull ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df
    fi
    
    while true; do
        show_menu
        read -p "Choose an option: " choice
        
        case $choice in
            1) test_full_workflow ;;
            2) handle_individual ;;
            3) 
                echo "Checking all validators..."
                check_node_status $CHARLIE_PORT "Charlie" 2>/dev/null || echo "Charlie not running"
                check_node_status $DAVE_PORT "Dave" 2>/dev/null || echo "Dave not running"
                check_node_status $EVE_PORT "Eve" 2>/dev/null || echo "Eve not running"
                ;;
            4)
                echo "Generating keys for all running validators..."
                generate_session_keys $CHARLIE_PORT "charlie" 2>/dev/null || echo "Charlie not running"
                generate_session_keys $DAVE_PORT "dave" 2>/dev/null || echo "Dave not running"
                generate_session_keys $EVE_PORT "eve" 2>/dev/null || echo "Eve not running"
                ;;
            5) create_registration_summary ;;
            6) cleanup_all ;;
            7) echo "👋 Goodbye!"; exit 0 ;;
            *) echo "❌ Invalid choice" ;;
        esac
        
        read -p "Press Enter to continue..."
    done
}

# Handle command line arguments
if [ $# -gt 0 ]; then
    case $1 in
        --test-all) test_full_workflow ;;
        --cleanup) cleanup_all ;;
        --summary) create_registration_summary ;;
        --help) 
            echo "Usage: $0 [--test-all|--cleanup|--summary|--help]"
            echo "  --test-all: Run complete testing workflow"
            echo "  --cleanup:  Clean up all test validators"
            echo "  --summary:  Show registration summary"
            echo "  --help:     Show this help"
            ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
else
    main
fi 