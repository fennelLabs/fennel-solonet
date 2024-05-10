/app/fennel-node key insert --key-type aura --base-path /app/chain --chain /app/fennelSpecRaw.json --scheme Sr25519 --suri "$chainsuri"
/app/fennel-node key insert --key-type gran --base-path /app/chain --chain /app/fennelSpecRaw.json --scheme Ed25519 --suri "$chainsuri"
/app/fennel-node --base-path /app/chain --chain /app/fennelSpecRaw.json --port 30333 --rpc-port 9945 --node-key 0000000000000000000000000000000000000000000000000000000000000001 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --validator --rpc-cors all --prometheus-external --rpc-methods Unsafe --rpc-external
