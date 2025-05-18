#!/bin/bash
cd /home/neurosx/WORKING_WORKSPACE/fennel-solonet-2503UPGRADE
./target/release/fennel-node \
  --base-path /tmp/bob \
  --chain local \
  --bob \
  --port 30334 \
  --rpc-port 9945 \
  --rpc-external \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --validator \
  --unsafe-force-node-key-generation \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWPM7VJvxqjabqPjgFR9pAnJi5vmr2XrypAUduY3CX1Ytq