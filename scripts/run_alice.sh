#!/bin/bash
cd /home/neurosx/WORKING_WORKSPACE/fennel-solonet-2503UPGRADE
./target/release/fennel-node \
  --base-path /tmp/alice \
  --chain local \
  --alice \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-external \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --validator \
  --unsafe-force-node-key-generation 