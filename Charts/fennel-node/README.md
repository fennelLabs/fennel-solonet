# Fennel Node Helm Chart (Production)

This Helm chart deploys a Fennel node on Kubernetes for the production Fennel chain. It is production-only and ships a single `values.yaml` for launching your own node.

## Prerequisites

- Kubernetes 1.23+
- Helm 3.13+
- PV provisioner in your cluster (for persistent storage)

## Install

Add repository and install:

```bash
helm repo add fennel https://corruptedaesthetic.github.io/fennel-solonet
helm repo update

helm install fennel-node fennel/fennel-node \
  --namespace fennel \
  --create-namespace
```

The chart defaults to connecting to the production network via the released `production-raw.json` chainspec. Override any field in `values.yaml` as needed.

## Key values

- `image.repository` / `image.tag`: Docker image for `fennel-node` (CI sets tag on release)
- `node.chain`: fixed to `production`
- `node.customChainspecUrl`: defaults to the latest published `production-raw.json`
- `node.chainData.storageClass` and `node.chainData.volumeSize`: persistent storage
- `node.perNodeServices`: exposes API (RPC/metrics) and P2P

Example overrides:

```bash
helm upgrade --install fennel-node fennel/fennel-node \
  -n fennel \
  --set node.chainData.storageClass=managed-csi \
  --set node.chainData.volumeSize=256Gi
```

## License

Apache-2.0