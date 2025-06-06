#!/bin/bash

# External Validator Testing Tool (Charlie, Dave, Eve)
# Tests the complete external validator onboarding workflow
#
# ✅ UPDATED: Now uses Docker image with ValidatorManager SessionManager genesis fix
#    Current: sha-2ea7777... (NEW - contains ValidatorManager genesis fix + cleanup)
#    Previous: sha-204fa8e... (OLD - missing ValidatorManager SessionManager fix)
#
# This script demonstrates:
# - Starting external validators with proper fennel-node configuration
# - Session key generation and verification
# - ValidatorManager registration workflow
# - Health monitoring and connectivity testing

set -e

echo "👥 Testing Charlie, Dave & Eve as External Validators"
echo "===================================================="
echo "✅ Using Docker image: ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-2ea7777df54a4bc1d113591d6a2351930bae3806"
echo "🔧 NEW: Contains ValidatorManager SessionManager genesis fix + repository cleanup"
echo ""

# Well-known development account IDs
CHARLIE_ACCOUNT="5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"
DAVE_ACCOUNT="5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
EVE_ACCOUNT="5HGjWAeFDfFCWPsjFQdVV2Mspz2XtMktvgocEZcCj68kUMaw"

# Port assignments for testing
CHARLIE_PORT=9946
DAVE_PORT=9947
EVE_PORT=9948

# Function to start test validator (FINAL VERSION)
start_test_validator() {
    local name=$1
    local port=$2
    local container_name="fennel-test-$name"
    
    echo "🚀 Starting $name as external validator (port $port)..."
    
    # Stop any existing container
    docker stop "$container_name" 2>/dev/null || true
    docker rm "$container_name" 2>/dev/null || true
    
    # Create data directory with proper permissions
    local data_dir="/tmp/fennel-test-$name"
    mkdir -p "$data_dir"
    chmod 777 "$data_dir"  # Ensure Docker can write to it
    
    # Use fennel-node with proper development arguments
    # The --dev flag enables: --chain=dev, --force-authoring, --rpc-cors=all, --alice
    docker run -d \
        --name "$container_name" \
        -p "$port:9944" \
        -p "$((port + 100)):30333" \
        -v "$data_dir:/data" \
        --user "$(id -u):$(id -g)" \
        ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-2ea7777df54a4bc1d113591d6a2351930bae3806 \
        --name "Test$name" \
        --base-path /data \
        --dev \
        --rpc-external \
        --rpc-methods Safe \
        --rpc-max-connections 100
    
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
    echo "🔄 Process Overview (using --dev flag):"
    echo "1. Connect to Alice via: kubectl port-forward -n fennel fennel-solochain-node-0 9944:9944"
    echo "2. Open Polkadot.js Apps and connect to ws://localhost:9944 (Alice's node)"
    echo "3. For each validator below, follow the session key process:"
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
        echo "  🔗 Polkadot.js Direct: ws://localhost:$port"
        echo ""
    done
    
    echo "🎯 Step-by-Step Validator Registration:"
    echo "1. Connect Polkadot.js Apps to Alice: ws://localhost:9944"
    echo "2. For each validator above:"
    echo "   a. Generate session keys: Use the curl command shown above"
    echo "   b. Set session keys on-chain:"
    echo "      - Go to Developer > Extrinsics"
    echo "      - Select validator's account (e.g., Charlie) as sender"  
    echo "      - Call: session.setKeys(session_keys, 0x00)"
    echo "      - Submit transaction"
    echo "3. Register all validators at once:"
    echo "   a. Go to Developer > Sudo"
    echo "   b. Call: validatorManager.registerValidators([account_ids])"
    echo "   c. Use Alice (sudo) account to submit"
    echo "4. Wait 1-2 session rotations for activation"
    echo "5. Verify with: session.validators() and validatorManager.validators()"
    echo ""
    echo "🎉 The --dev flag enables: --chain=dev, --force-authoring, --rpc-cors=all"
    echo "This should resolve the CORS issues when connecting from Polkadot.js Apps!"
}

# Function to test full workflow
test_full_workflow() {
    echo "🎯 Testing Full External Validator Workflow (FINAL)"
    echo "=================================================="
    
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
    echo "🎮 External Validator Testing Menu (FINAL)"
    echo "=========================================="
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
    echo "👥 External Validator Testing Tool (FINAL)"
    echo "=========================================="
    echo "This tool helps test Charlie, Dave & Eve as external validators"
    echo "Using proper fennel-node arguments with --dev flag"
    echo ""
    
    # Check if Docker is available
    if ! command -v docker &> /dev/null; then
        echo "❌ Docker is not available. Please install Docker first."
        exit 1
    fi
    
    # Check if image exists
    if ! docker image inspect ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-2ea7777df54a4bc1d113591d6a2351930bae3806 &> /dev/null; then
        echo "⚠️  Fennel Docker image not found locally. Pulling..."
        docker pull ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-2ea7777df54a4bc1d113591d6a2351930bae3806
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