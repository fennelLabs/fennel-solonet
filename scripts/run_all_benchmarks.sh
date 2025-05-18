#!/bin/bash

# Set variables
RUNTIME_WASM="/home/neurosx/WORKING_WORKSPACE/fennel-solonet-2503UPGRADE/target/release/wbuild/fennel-node-runtime/fennel_node_runtime.wasm"
TEMPLATE="/home/neurosx/WORKING_WORKSPACE/fennel-solonet-2503UPGRADE/scripts/benchmarking/frame-weight-template.hbs"
BENCHMARK_DIR="/home/neurosx/WORKING_WORKSPACE/fennel-solonet-2503UPGRADE/benchmarks_output"

# Create output directory
mkdir -p $BENCHMARK_DIR

# List of pallets to benchmark
PALLETS=(
  "pallet_certificate"
  "pallet_identity"
  "pallet_infostratus"
  "pallet_keystore"
  "pallet_signal"
  "pallet_trust"
  "pallet_validator_manager"
)

# Check if runtime exists
if [ ! -f "$RUNTIME_WASM" ]; then
  echo "Runtime WASM not found at $RUNTIME_WASM"
  echo "Make sure you've compiled the runtime with: cargo build --release --features runtime-benchmarks"
  exit 1
fi

# Check if template exists
if [ ! -f "$TEMPLATE" ]; then
  echo "Template file not found at $TEMPLATE"
  exit 1
fi

# Function to run benchmark for a pallet
run_benchmark() {
  local pallet=$1
  local output_file="$BENCHMARK_DIR/${pallet}_weights.rs"
  
  echo "Running benchmark for $pallet..."
  echo "Output will be saved to $output_file"
  
  frame-omni-bencher v1 benchmark pallet \
    --runtime "$RUNTIME_WASM" \
    --pallet "$pallet" \
    --extrinsic "" \
    --template "$TEMPLATE" \
    --output "$output_file"
    
  if [ $? -eq 0 ]; then
    echo "Successfully generated weights for $pallet"
  else
    echo "Failed to generate weights for $pallet"
  fi
  
  echo "----------------------------------------"
}

# Run benchmarks for all pallets
echo "Starting benchmarks for all pallets..."
for pallet in "${PALLETS[@]}"; do
  run_benchmark "$pallet"
done

echo "All benchmarks completed. Results are in $BENCHMARK_DIR" 