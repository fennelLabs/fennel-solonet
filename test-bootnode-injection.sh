#!/bin/bash

# Test script for bootnode injection functionality
# This simulates the GitHub Actions workflow steps locally

set -euo pipefail

echo "🧪 Testing Bootnode Injection Functionality"
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
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

# Check prerequisites
print_status "Checking prerequisites..."

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    print_error "jq is not installed. Please install it first:"
    echo "  - Ubuntu/Debian: sudo apt-get install jq"
    echo "  - macOS: brew install jq"
    exit 1
fi
print_success "jq is available"

# Check if Docker is running
if ! docker info &> /dev/null; then
    print_error "Docker is not running. Please start Docker first."
    exit 1
fi
print_success "Docker is available"

# Create test directory structure
print_status "Setting up test environment..."
mkdir -p test-output/chainspecs/{development,staging}

# Create a mock chain spec for testing
print_status "Creating mock chain spec..."
cat > test-output/chainspecs/development/development.json << 'EOF'
{
  "name": "Fennel Development",
  "id": "fennel_dev",
  "chainType": "Development",
  "genesis": {
    "raw": {
      "top": {}
    }
  },
  "properties": {
    "tokenDecimals": 18,
    "tokenSymbol": "FNL"
  },
  "consensusEngine": null,
  "lightSyncState": null,
  "codeSubstitutes": {},
  "forkBlocks": null,
  "badBlocks": null,
  "telemetryEndpoints": null
}
EOF

print_success "Mock chain spec created"

# Test the bootnode injection logic
print_status "Testing bootnode injection..."

SPEC="test-output/chainspecs/development/development.json"

# Show the chain spec BEFORE bootnode injection
echo -e "\n${YELLOW}📋 BEFORE: Chain spec structure (keys only):${NC}"
jq 'keys' "$SPEC"

# Define bootnode multiaddresses using the peer IDs
BOOTNODES=(
  "/dns/bootnode1.fennel.network/tcp/30333/p2p/12D3KooWDCZGrnJhsgWJtDcs6eZc1hUBrVj5QqkEWggAkNVowRAi"
  "/dns/bootnode2.fennel.network/tcp/30333/p2p/12D3KooWDbfFv6oepAXmQaiwFaVjD9g7AxowQ8LQdWTcVYpKhnAx"
)

print_status "Preparing bootnode injection..."
echo "Bootnodes to inject:"
for i in "${!BOOTNODES[@]}"; do
    echo "  $((i+1)). ${BOOTNODES[$i]}"
done

# Build a JSON array string
JQ_ARRAY=$(printf '%s\n' "${BOOTNODES[@]}" | jq -R . | jq -s .)

# Inject the bootNodes field into the spec
jq --argjson arr "$JQ_ARRAY" '.bootNodes = $arr' "$SPEC" > tmp.json && mv tmp.json "$SPEC"

print_success "Bootnode injection completed"

# Show the chain spec AFTER bootnode injection
echo -e "\n${GREEN}📋 AFTER: Chain spec structure (keys only):${NC}"
jq 'keys' "$SPEC"

echo -e "\n${GREEN}✨ NEW: bootNodes field content:${NC}"
jq '.bootNodes' "$SPEC"

# Verify the injection worked
print_status "Verifying bootnode injection..."

# Check if bootNodes field exists and has correct content
BOOTNODE_COUNT=$(jq '.bootNodes | length' "$SPEC")
if [ "$BOOTNODE_COUNT" -eq 2 ]; then
    print_success "Correct number of bootnodes found: $BOOTNODE_COUNT"
else
    print_error "Expected 2 bootnodes, found: $BOOTNODE_COUNT"
    exit 1
fi

# Verify each bootnode
echo "Verifying bootnode addresses:"
jq -r '.bootNodes[]' "$SPEC" | while read -r bootnode; do
    if [[ $bootnode =~ ^/dns/bootnode[12]\.fennel\.network/tcp/30333/p2p/12D3KooW[A-Za-z0-9]+$ ]]; then
        print_success "Valid bootnode: $bootnode"
    else
        print_error "Invalid bootnode format: $bootnode"
        exit 1
    fi
done

# Display detailed results with before/after comparison
print_status "Detailed bootnode injection results:"
echo "=========================================="

echo -e "\n${BLUE}📋 Chain Spec Overview:${NC}"
echo "Name: $(jq -r '.name' "$SPEC")"
echo "ID: $(jq -r '.id' "$SPEC")"
echo "Chain Type: $(jq -r '.chainType' "$SPEC")"

echo -e "\n${BLUE}🔗 Bootnode Details:${NC}"
BOOTNODE_COUNT=$(jq '.bootNodes | length' "$SPEC")
echo "Total bootnodes: $BOOTNODE_COUNT"

echo -e "\n${BLUE}📍 Individual Bootnodes:${NC}"
jq -r '.bootNodes[]' "$SPEC" | nl -v0 -s': ' | while read -r line; do
    echo "  $line"
done

echo -e "\n${BLUE}🔍 Bootnode Analysis:${NC}"
jq -r '.bootNodes[]' "$SPEC" | while read -r bootnode; do
    # Extract components
    if [[ $bootnode =~ ^/dns/([^/]+)/tcp/([0-9]+)/p2p/([A-Za-z0-9]+)$ ]]; then
        hostname="${BASH_REMATCH[1]}"
        port="${BASH_REMATCH[2]}"
        peer_id="${BASH_REMATCH[3]}"
        echo "  ✅ Valid bootnode:"
        echo "     - Hostname: $hostname"
        echo "     - Port: $port"
        echo "     - Peer ID: $peer_id"
        echo ""
    else
        echo "  ❌ Invalid bootnode format: $bootnode"
    fi
done

echo -e "\n${BLUE}📄 Complete Chain Spec (Pretty Formatted):${NC}"
echo "=============================================="
jq '.' "$SPEC"

echo -e "\n${BLUE}💾 Chain Spec saved temporarily to: ${NC}$SPEC"

print_success "Bootnode injection test completed successfully!"

# Test chain-spec-builder installation (optional)
print_status "Testing chain-spec-builder installation..."
if command -v chain-spec-builder &> /dev/null; then
    print_success "chain-spec-builder is already installed"
    chain-spec-builder --version || print_warning "Version check failed"
else
    print_warning "chain-spec-builder not found locally"
    print_status "Testing installation via Docker..."
    
    # Test the Docker installation command from the workflow
    docker run --rm \
        -v "${PWD}":/build \
        -v "${HOME}/.cargo/bin":/cargo-bin \
        --workdir /build \
        paritytech/ci-unified:latest \
        bash -c "
            echo 'Testing chain-spec-builder installation...' && \
            cargo install staging-chain-spec-builder --locked --root /tmp/cargo-install && \
            /tmp/cargo-install/bin/chain-spec-builder --version || echo 'Version check failed'
        " && print_success "Docker-based installation test passed" || print_error "Docker-based installation test failed"
fi

# Create mock runtime for testing
print_status "Creating mock runtime WASM for testing..."
mkdir -p runtime/fennel/target/srtool/release/wbuild/fennel-node-runtime
echo -e '\x00asm\x01\x00\x00\x00' > runtime/fennel/target/srtool/release/wbuild/fennel-node-runtime/fennel_node_runtime.compact.wasm
print_success "Mock runtime WASM created"

# Test with mock runtime
if [ -f "runtime/fennel/target/srtool/release/wbuild/fennel-node-runtime/fennel_node_runtime.compact.wasm" ]; then
    print_status "Testing chain-spec-builder with mock runtime..."
    
    # Note: This will likely fail since it's a mock WASM, but we can test the file detection
    WASM_SIZE=$(stat -c%s "runtime/fennel/target/srtool/release/wbuild/fennel-node-runtime/fennel_node_runtime.compact.wasm")
    print_success "Mock runtime WASM found (${WASM_SIZE} bytes)"
    
    print_warning "Mock runtime testing note:"
    echo "  - File exists and is detected ✅"
    echo "  - chain-spec-builder would fail with mock WASM (expected)"
    echo "  - In CI, real runtime will be built with srtool"
    echo ""
    echo "To test with real runtime locally:"
    echo "1. Build runtime: docker run --rm -v \"\${PWD}\":/build -e RUNTIME_DIR=runtime/fennel -e PACKAGE=fennel-node-runtime --workdir /build paritytech/srtool:1.84.1 /srtool/build"
    echo "2. Run: chain-spec-builder -c test-output/real-spec.json create -r runtime/fennel/target/srtool/release/wbuild/fennel-node-runtime/fennel_node_runtime.compact.wasm named-preset development"
else
    print_error "Failed to create mock runtime WASM"
fi

# Pause before cleanup to allow inspection
echo ""
echo "🔍 INSPECTION PAUSE"
echo "==================="
echo "The test files are ready for inspection:"
echo "  - Chain spec with bootnodes: test-output/chainspecs/development/development.json"
echo "  - Mock runtime WASM: runtime/fennel/target/srtool/release/wbuild/fennel-node-runtime/fennel_node_runtime.compact.wasm"
echo ""
echo "You can:"
echo "  - Inspect the files manually"
echo "  - Copy them for further analysis"
echo "  - Verify the JSON structure"
echo ""
read -p "Press Enter to continue with cleanup, or Ctrl+C to stop and inspect files..."

# Cleanup
print_status "Cleaning up test environment..."
rm -rf test-output
rm -rf runtime/fennel/target/srtool  # Remove mock runtime
print_success "Cleanup completed"

echo ""
echo "🎉 All tests passed! The bootnode injection should work in GitHub Actions."
echo ""
echo "Next steps:"
echo "1. Push your changes to trigger the workflow"
echo "2. Monitor the GitHub Actions logs for the bootnode injection steps"
echo "3. Check the generated chainspecs in the artifacts" 