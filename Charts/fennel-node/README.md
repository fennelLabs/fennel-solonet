# Fennel Node Helm Chart

This Helm chart deploys Fennel blockchain validator nodes on Kubernetes, following Polkadot SDK ecosystem standards.

## Overview

The chart uses Parity's official node chart as a dependency and provides Fennel-specific configurations for both development and staging environments.

## Prerequisites

- Kubernetes 1.23+
- Helm 3.13+
- PV provisioner support in the underlying infrastructure (for persistent storage)

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
| `node.chainSpecConfigMap` | ConfigMap containing chain spec | `fennel-chainspec` |
| `node.chainData.volumeSize` | Size of persistent volume | `100Gi` |
| `node.chainData.storageClass` | Storage class for PV | `fast-ssd` |
| `node.chainSnapshot.enabled` | Enable snapshot restoration | `false` (true in staging) |
| `node.serviceMonitor.enabled` | Enable Prometheus monitoring | `false` (true in staging) |

### Staging Environment Features

The staging configuration enables:

1. **Persistent Storage**: 100Gi volume with fast SSD storage class
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
# Run the test script
./scripts/test-helm-chart.sh

# Or manually:
helm dependency update
helm lint .
helm lint . -f values-staging.yaml
helm template fennel-test . -f values-staging.yaml
```

## Upgrading

To upgrade an existing release:

```bash
helm upgrade fennel-staging fennel/fennel-node \
  --namespace fennel-staging \
  -f values-staging.yaml
```

## Uninstallation

```bash
helm uninstall fennel-staging --namespace fennel-staging
```

## Troubleshooting

### Check pod status
```bash
kubectl get pods -n fennel-staging
kubectl describe pod <pod-name> -n fennel-staging
```

### View logs
```bash
kubectl logs -n fennel-staging -l app=fennel-node
```

### Check persistent volume
```bash
kubectl get pvc -n fennel-staging
```

## License

This chart is licensed under the Apache License 2.0. 