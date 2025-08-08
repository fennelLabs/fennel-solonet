# Fennel Protocol - Solonet

Solochain for the Fennel network, built on the Polkadot SDK. This repo contains:
- Fennel node implementation (`fennel-node`)
- Fennel runtime (`runtime/fennel`)
- Helm chart to deploy production nodes (`Charts/fennel-node`)
- CI to build Docker images and generate chainspecs (development, staging, production)

## Build

```sh
cargo build --release
./target/release/fennel-node -h
```

## Run a local development node

```sh
./target/release/fennel-node --dev
# Purge local state
./target/release/fennel-node purge-chain --dev
```

Key files:
- Chain spec source: `node/src/chain_spec.rs`
- Runtime: `runtime/fennel/src/lib.rs`

Connect with Polkadot.js Apps locally: `ws://127.0.0.1:9944`

## Chainspecs

CI builds and attaches chainspecs to releases:
- Development: `chainspecs/development/*`
- Staging: `chainspecs/staging/*`
- Production: `chainspecs/production/production-raw.json`

Production chainspec and Docker image are built in the same tagged run (`fennel-node-X.Y.Z`). The Helm chart values are updated with the exact image tag and the production chainspec SHA-256 for verification.

## Helm (production node)

```sh
helm repo add fennel https://corruptedaesthetic.github.io/fennel-solonet
helm repo update

helm upgrade --install fennel-node fennel/fennel-node \
  --namespace fennel \
  --create-namespace
```

Defaults:
- `node.chain: production`
- `node.customChainspecUrl`: latest `production-raw.json` from releases
- Metrics enabled; ServiceMonitor disabled by default

Override examples:

```sh
helm upgrade --install fennel-node fennel/fennel-node \
  -n fennel \
  --set node.chainData.storageClass=managed-csi \
  --set node.chainData.volumeSize=256Gi
```

## Releases

Tag with `fennel-node-X.Y.Z` to build and publish:
- Docker images (multi-arch: linux/amd64 and linux/arm64): `ghcr.io/<org>/fennel-solonet:fennel-node-X.Y.Z`
- Helm chart (version/appVersion set to `X.Y.Z`)
- Chainspecs (dev, staging, production)
- Node binaries for automation:
  - Raw binary suitable for Parity Ansible (with SHA256 checksum)
  - Tarball (`.tar.gz`) with SHA256 checksum

## License

Apache-2.0
