#!/bin/bash

# Set default values
steps=${1:-50}
repeat=${2:-20}

# Print usage information
echo "Running benchmarks with steps=$steps and repeat=$repeat"
echo "If you want to change these values, run: $0 <steps> <repeat>"
echo "Example: $0 100 30"

# Create output directory if it doesn't exist
mkdir -p ./runtime/fennel/src/weights

# All pallets to benchmark
pallets=(
    pallet_validator_manager
    pallet_trust
    pallet_signal
    pallet_keystore
    pallet_identity
    pallet_infostratus
    pallet_certificate
)

echo "Building release version with runtime benchmarks enabled..."
cargo build --release --features=runtime-benchmarks --bin=fennel-node

for p in "${pallets[@]}"
do
  echo "Running benchmarks for $p with steps=$steps and repeat=$repeat"
  ./target/release/fennel-node benchmark pallet \
    --chain=dev \
    --wasm-execution=compiled \
    --pallet=$p \
    --extrinsic='*' \
    --steps=$steps \
    --repeat=$repeat \
    --template=./scripts/templates/weight-template.hbs \
    --output=./runtime/fennel/src/weights
  
  # Check if benchmarking was successful and copy the generated file to the respective pallet
  # Handle special case for validator-manager (underscore vs hyphen)
  if [ "$p" == "pallet_validator_manager" ]; then
    pallet_path="validator-manager"
  else
    pallet_path=$(echo $p | cut -d_ -f2-)
  fi
  
  if [ -f "./runtime/fennel/src/weights/${p}.rs" ]; then
    echo "Benchmarking completed successfully for $p. Weight file created."
    echo "Copying to pallets/${pallet_path}/src/weights.rs"
    cp "./runtime/fennel/src/weights/${p}.rs" "./pallets/${pallet_path}/src/weights.rs"
  else
    echo "Benchmarking failed or weight file was not created for $p."
  fi
done

echo "All benchmarks completed."
echo "To use these weights, ensure each pallet has its weights properly configured in its Config trait." 