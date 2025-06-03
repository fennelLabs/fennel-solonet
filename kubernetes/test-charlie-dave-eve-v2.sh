#!/bin/bash

# Test Charlie, Dave, and Eve as External Validators (V2 - Try different chain specs)
# This version tries multiple chain specifications to find one that works

set -e

echo "👥 Testing Charlie, Dave & Eve as External Validators (V2)"
echo "=========================================================="

# Port assignments for testing
CHARLIE_PORT=9946

# Function to test different chain configurations
test_chain_config() {
    local config_name=$1
    local chain_arg=$2
    local container_name="fennel-test-charlie-$config_name"
    
    echo "🚀 Testing $config_name configuration..."
    
    # Stop any existing container
    docker stop "$container_name" 2>/dev/null || true
    docker rm "$container_name" 2>/dev/null || true
    
    # Create data directory
    mkdir -p "/tmp/fennel-test-charlie-$config_name"
    
    # Build docker command
    local docker_cmd="docker run -d --name $container_name -p $CHARLIE_PORT:9944 -v /tmp/fennel-test-charlie-$config_name:/data ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df --name TestCharlie --base-path /data"
    
    if [ -n "$chain_arg" ]; then
        docker_cmd="$docker_cmd $chain_arg"
    fi
    
    echo "  Command: $docker_cmd"
    
    # Run the container
    eval $docker_cmd
    
    # Wait a moment for startup
    sleep 5
    
    # Check if it's running and responsive
    if docker ps | grep -q "$container_name"; then
        echo "  ✅ Container is running"
        
        # Test RPC endpoint
        if curl -s -H "Content-Type: application/json" \
            -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
            "http://localhost:$CHARLIE_PORT" >/dev/null 2>&1; then
            echo "  ✅ RPC is responding!"
            echo "  🎉 SUCCESS with $config_name configuration!"
            
            # Show logs for verification
            echo "  📋 Container logs:"
            docker logs "$container_name" | tail -10
            
            return 0
        else
            echo "  ❌ RPC not responding"
        fi
    else
        echo "  ❌ Container failed to start"
    fi
    
    # Show logs for debugging
    echo "  📋 Container logs:"
    docker logs "$container_name" 2>&1 | tail -10
    
    # Cleanup
    docker stop "$container_name" 2>/dev/null || true
    docker rm "$container_name" 2>/dev/null || true
    
    return 1
}

# Function to cleanup all test containers
cleanup_all() {
    echo "🧹 Cleaning up all test containers..."
    docker ps -a | grep "fennel-test-charlie-" | awk '{print $1}' | xargs -r docker rm -f
    rm -rf /tmp/fennel-test-charlie-*
    echo "  ✅ Cleanup complete"
}

# Main testing function
main() {
    echo "🎯 Testing Different Chain Configurations"
    echo "========================================"
    echo ""
    
    # Test configurations in order of likelihood to work
    local configs=(
        "no-chain:"
        "local:--chain local"
        "dev:--chain dev"
        "development:--chain development"
        "local-testnet:--chain local-testnet"
    )
    
    for config in "${configs[@]}"; do
        local name="${config%%:*}"
        local args="${config#*:}"
        
        echo "Testing configuration: $name"
        echo "Arguments: $args"
        echo ""
        
        if test_chain_config "$name" "$args"; then
            echo ""
            echo "🎉 FOUND WORKING CONFIGURATION: $name"
            echo "🔗 You can now connect Polkadot.js to: ws://localhost:$CHARLIE_PORT"
            echo ""
            echo "📋 To use this configuration manually:"
            echo "docker run -d --name fennel-test-charlie \\"
            echo "  -p $CHARLIE_PORT:9944 \\"
            echo "  -v /tmp/fennel-test-charlie:/data \\"
            echo "  ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df \\"
            echo "  --name TestCharlie \\"
            echo "  --base-path /data \\"
            if [ -n "$args" ]; then
                echo "  $args"
            fi
            echo ""
            echo "Container is still running for testing. Use 'docker stop fennel-test-charlie-$name' to stop it."
            return 0
        fi
        
        echo ""
    done
    
    echo "❌ None of the configurations worked."
    echo "The fennel-node binary may have a different interface than expected."
    echo ""
    echo "🔍 Let's check what chain specs are available..."
    
    # Try to get help output
    docker run --rm ghcr.io/corruptedaesthetic/uptodatefennelnetmp:sha-204fa8e5891442d07ab060fb2ff7301703b5a4df --help 2>&1 | grep -i chain || true
    
    return 1
}

# Handle command line arguments
case "${1:-}" in
    --test)
        main
        ;;
    --cleanup)
        cleanup_all
        ;;
    --help|*)
        echo "Usage: $0 [--test|--cleanup|--help]"
        echo "  --test:    Test different chain configurations"
        echo "  --cleanup: Clean up all test containers"
        echo "  --help:    Show this help"
        ;;
esac 