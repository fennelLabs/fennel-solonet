#!/bin/bash

steps=$1
repeat=$2

pallets=(
    pallet_sudo
    pallet_balances
    pallet_certificate
    pallet_identity
    pallet_keystore
    pallet_signal
    pallet_trust
    pallet_infostratus
)

cargo build --release --features=runtime-benchmarks --bin=fennel-node

for p in ${pallets[@]}
do
  ./target/release/fennel-node benchmark pallet \
    --chain=dev \
    --wasm-execution=compiled \
    --pallet=$p \
    --extrinsic='*' \
    --steps=$steps \
    --repeat=$repeat \
    --template=./scripts/templates/weight-template.hbs \
    --output=./runtime/src/weights
done
