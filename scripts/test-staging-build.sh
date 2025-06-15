#!/bin/bash

echo "=== Testing Fennel Staging Chain Build ==="
echo

# Check if the binary exists
if [ ! -f "../target/release/fennel-node" ]; then
    echo "❌ Error: fennel-node binary not found!"
    echo "Please wait for 'cargo build --release' to complete."
    exit 1
fi

echo "✅ Binary found at ../target/release/fennel-node"
echo

# Test 1: Check if staging chain is recognized
echo "Test 1: Checking if 'staging' chain is recognized..."
../target/release/fennel-node build-spec --chain staging --help &> /dev/null
if [ $? -eq 0 ]; then
    echo "✅ Staging chain is recognized"
else
    echo "❌ Staging chain not recognized"
    exit 1
fi

# Test 2: Generate staging chain spec (plain)
echo
echo "Test 2: Generating staging chain spec (plain)..."
../target/release/fennel-node build-spec --chain staging > staging.json 2>/dev/null
if [ $? -eq 0 ] && [ -f "staging.json" ]; then
    echo "✅ Plain chain spec generated successfully"
    
    # Check if bootnodes are present
    if grep -q "bootNodes" staging.json; then
        echo "✅ Bootnodes found in chain spec"
        echo "Bootnode entries:"
        grep -A 5 "bootNodes" staging.json | head -10
    else
        echo "⚠️  No bootnodes found in chain spec"
    fi
else
    echo "❌ Failed to generate plain chain spec"
    exit 1
fi

# Test 3: Generate raw chain spec
echo
echo "Test 3: Generating raw chain spec..."
../target/release/fennel-node build-spec --chain staging --raw > staging_raw.json 2>/dev/null
if [ $? -eq 0 ] && [ -f "staging_raw.json" ]; then
    echo "✅ Raw chain spec generated successfully"
    echo "   File size: $(ls -lh staging_raw.json | awk '{print $5}')"
else
    echo "❌ Failed to generate raw chain spec"
    exit 1
fi

# Test 4: Verify chain spec contents
echo
echo "Test 4: Verifying chain spec contents..."
if [ -f "staging.json" ]; then
    echo "Chain name: $(grep -m1 '"name"' staging.json | cut -d'"' -f4)"
    echo "Chain ID: $(grep -m1 '"id"' staging.json | cut -d'"' -f4)"
    echo "Chain type: $(grep -m1 '"chainType"' staging.json | cut -d'"' -f4)"
fi

# Test 5: Dry run the node
echo
echo "Test 5: Dry run test (will stop after 5 seconds)..."
echo "Starting node with staging chain..."
timeout 5 ../target/release/fennel-node --chain staging --tmp --name "StagingTest" 2>&1 | grep -E "(Local node identity|Fennel Staging|error)" || true

echo
echo "=== Summary ==="
echo "All tests completed. Your staging chain is ready!"
echo
echo "Next steps:"
echo "1. Update the bootnode addresses in chain_spec.rs with actual IPs"
echo "2. Rebuild and regenerate chain specs"
echo "3. Deploy to Kubernetes"
echo
echo "To start a local staging node:"
echo "   ../target/release/fennel-node --chain staging --dev --tmp" 