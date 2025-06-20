# Fennel Node Helm Chart

This Helm chart deploys Fennel blockchain validator nodes on Kubernetes, following Polkadot SDK ecosystem standards.

## Overview

The chart uses Parity's official node chart (v5.15.0) as a dependency and provides Fennel-specific configurations for both development and staging environments.

## Prerequisites

- Kubernetes 1.23+
- Helm 3.13+
- PV provisioner support in the underlying infrastructure (for persistent storage)
- A configured StorageClass (e.g., `local-path`, `fast-ssd`)

## Installation

### Add the Helm repository

```bash
helm repo add fennel https://corruptedaesthetic.github.io/fennel-solonet
helm repo update
```

### Install the chart

For development environment:
```bash
helm install fennel-dev fennel/fennel-node \
  --namespace fennel-dev \
  --create-namespace
```

For staging environment:
```bash
helm install fennel-staging fennel/fennel-node \
  --namespace fennel-staging \
  --create-namespace \
  -f values-staging.yaml
```

## Configuration

The chart provides two configuration files:

- `values.yaml` - Base configuration with development defaults
- `values-staging.yaml` - Production-grade settings for staging environment

### Key Configuration Options

| Parameter | Description | Default |
|-----------|-------------|---------|
| `image.repository` | Docker image repository | `ghcr.io/corruptedaesthetic/fennel-solonet` |
| `image.tag` | Docker image tag | Set by CI pipeline |
| `chainspec.file` | Chain specification file | `development.json` |
| `node.image.repository` | Node image repository (overrides base image) | Same as `image.repository` |
| `node.node.chainData.storageClass` | Storage class for chain data PV | `fast-ssd` |
| `node.node.chainData.volumeSize` | Size of persistent volume | `100Gi` |
| `node.node.chainSnapshot.enabled` | Enable snapshot restoration | `false` (true in staging) |
| `node.serviceMonitor.enabled` | Enable Prometheus monitoring | `false` (true in staging) |

### Major Changes in v0.2.0

**Breaking Changes**: This version upgrades from Parity node chart v0.10.0 to v5.15.0, which includes significant structural changes:

1. **Storage Class Configuration**: Now configured at `node.node.chainData.storageClass` instead of root level
2. **Service Configuration**: Services are now configured under `node.node.perNodeServices`
3. **Chainspec Mounting**: Custom chainspec is mounted via `extraVolumes` and `extraVolumeMounts`
4. **Node Keys**: Configured via `node.node.existingNodeKeySecret` and `node.node.keys`

### Staging Environment Features

The staging configuration enables:

1. **Persistent Storage**: 100Gi volume with local-path storage class
2. **Snapshot Restoration**: Automatic chain data restoration from snapshots
3. **Secure Key Management**: Integration with Kubernetes Secrets for validator keys
4. **Service Segmentation**: Separate services for API and P2P traffic
5. **Monitoring**: Prometheus ServiceMonitor for metrics collection

## CI/CD Integration

The chart is automatically packaged and published by the GitHub Actions workflow when changes are pushed to the main branch. The workflow:

1. Builds the Docker image with srtool
2. Generates chain specifications
3. Updates Helm values with the correct image tag
4. Lints and packages the chart
5. Publishes to GitHub Pages Helm repository

## Local Development

To test the chart locally:

```bash
# Update dependencies
helm dependency update Charts/fennel-node

# Run the test script
./scripts/test-helm-chart.sh

# Or manually:
helm lint Charts/fennel-node
helm lint Charts/fennel-node -f Charts/fennel-node/values-staging.yaml
helm template fennel-test Charts/fennel-node -f Charts/fennel-node/values-staging.yaml
```

## Upgrading from v0.1.x

When upgrading from v0.1.x to v0.2.x, note the following breaking changes:

1. Update your custom values files to use the new structure
2. Storage class is now at `node.node.chainData.storageClass`
3. Services configuration has moved to `node.node.perNodeServices`
4. Update any scripts or CI/CD pipelines that reference the old paths

## Troubleshooting

### Check pod status
```bash
kubectl get pods -n fennel-staging
kubectl describe pod <pod-name> -n fennel-staging
```

### View logs
```bash
kubectl logs -n fennel-staging -l app.kubernetes.io/name=node
```

### Check persistent volume
```bash
kubectl get pvc -n fennel-staging
kubectl describe pvc -n fennel-staging
```

### Verify storage class
```bash
kubectl get storageclass
kubectl describe storageclass local-path
```

## License

This chart is licensed under the Apache License 2.0. 