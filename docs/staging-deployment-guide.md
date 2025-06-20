june 16 2025
# Fennel Staging Deployment Guide

This guide explains how the fennel-solonet and fennel-prod repositories work together to deploy Fennel blockchain nodes to Kubernetes staging environments.

## Architecture Overview

```
┌─────────────────────┐         ┌──────────────────┐         ┌─────────────────┐
│   fennel-solonet    │  ──→    │   GitHub Pages   │  ←──    │   fennel-prod   │
│   (Source Code)     │         │   (Helm Repo)    │         │   (GitOps)      │
└─────────────────────┘         └──────────────────┘         └─────────────────┘
         │                               │                            │
         ├─ Builds Docker Images         ├─ Hosts Helm Charts         ├─ Deploys via Flux
         ├─ Generates Chain Specs        └─ fennel-node-x.x.x.tgz    └─ HelmRelease
         └─ Packages Helm Charts                                        
```

## Repository Responsibilities

### fennel-solonet (CI/CD)
- **Purpose**: Source code, build automation, and artifact generation
- **Key Outputs**:
  - Docker images → `ghcr.io/corruptedaesthetic/fennel-solonet:sha-XXXXXXX`
  - Helm charts → Published to GitHub Pages
  - Chain specs → Committed to repository

### fennel-prod (GitOps)
- **Purpose**: Declarative infrastructure configuration
- **Key Components**:
  - Flux CD configurations
  - HelmRelease definitions
  - Environment-specific overlays

## CI/CD Pipeline Flow

### 1. Source Code Changes (fennel-solonet)

When code is pushed to `main` branch:

1. **Docker Image Build**:
   ```
   - Builds runtime with srtool
   - Creates Docker image with fennel-node binary
   - Tags: sha-XXXXXXX (commit SHA)
   - Pushes to: ghcr.io/corruptedaesthetic/fennel-solonet
   ```

2. **Chain Spec Generation**:
   ```
   - Creates development and staging chain specs
   - Injects bootnodes configuration
   - Commits back to repository
   ```

3. **Helm Chart Publishing**:
   ```
   - Updates image tag in values.yaml
   - Packages chart (now v0.2.0 with Parity node v5.15.0)
   - Publishes to GitHub Pages via chart-releaser
   - Available at: https://corruptedaesthetic.github.io/fennel-solonet
   ```

### 2. GitOps Deployment (fennel-prod)

Flux CD continuously monitors and reconciles:

1. **HelmRepository**: Points to GitHub Pages
2. **HelmRelease**: Deploys specific chart version
3. **ConfigMaps**: Chain specifications
4. **Secrets**: Validator keys

## Current Configuration Issues & Fixes

### Issue 1: Outdated Chart Version
The fennel-prod HelmRelease is using chart version 0.1.3, but we've upgraded to 0.2.0.

**Fix**:
```yaml
# In fennel-prod/clusters/staging/helmrelease.yaml
spec:
  chart:
    spec:
      chart: fennel-node
      version: "0.2.0"  # Update from 0.1.3
```

### Issue 2: Values Structure Mismatch
The HelmRelease values need to be updated for the new Parity chart structure.

**Fix**:
Update the HelmRelease values to match the new structure. Here's the corrected configuration:

```yaml
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
metadata:
  name: fennel-staging
  namespace: fennel-staging
spec:
  interval: 5m
  chart:
    spec:
      chart: fennel-node
      version: "0.2.0"
      sourceRef:
        kind: HelmRepository
        name: fennel-charts
        namespace: flux-system
  values:
    # Image configuration
    image:
      repository: ghcr.io/corruptedaesthetic/fennel-solonet
      tag: "sha-XXXXXXX"  # Updated by CI
    
    # Chainspec configuration
    chainspec:
      file: "staging.json"
    
    # Configure the Parity node subchart
    node:
      enabled: true
      
      # Override image
      image:
        repository: ghcr.io/corruptedaesthetic/fennel-solonet
        tag: "sha-XXXXXXX"  # Same as above
        pullPolicy: Always
      
      # Node configuration
      node:
        chain: "fennel"
        command: "fennel-node"
        replicas: 1
        role: "authority"
        
        # Custom chainspec
        customChainspec: true
        customChainspecPath: "/chainspec/staging.json"
        
        # Storage configuration
        chainData:
          storageClass: "local-path"
          volumeSize: "100Gi"
        
        chainKeystore:
          storageClass: "local-path"
          volumeSize: "10Mi"
        
        # Node keys from secrets
        existingNodeKeySecret:
          enabled: true
          secretName: "node-key"
        
        keys:
          - secretName: "validator-keys"
        
        # Services
        perNodeServices:
          apiService:
            enabled: true
            type: ClusterIP
          p2pService:
            enabled: true
            type: NodePort
            nodePort: 30333
        
        # Prometheus metrics
        prometheus:
          enabled: true
          port: 9615
      
      # Additional volumes for chainspec
      extraVolumes:
        - name: chainspec
          configMap:
            name: "fennel-chainspec"
      
      extraVolumeMounts:
        - name: chainspec
          mountPath: /chainspec
          readOnly: true
      
      # Service monitor
      serviceMonitor:
        enabled: true
        interval: "30s"
      
      # Resources
      resources:
        requests:
          cpu: 500m
          memory: 512Mi
        limits:
          cpu: 1
          memory: 1Gi
      
      # Additional environment variables
      extraEnvVars:
        - name: RUST_LOG
          value: "info"
```

## Deployment Steps

### 1. Update fennel-prod Repository

```bash
# Clone fennel-prod
git clone https://github.com/your-org/fennel-prod
cd fennel-prod

# Update HelmRelease
# Edit clusters/staging/helmrelease.yaml with the fixes above

# Commit and push
git add clusters/staging/helmrelease.yaml
git commit -m "Update staging to use fennel-node chart v0.2.0"
git push origin main
```

### 2. Monitor Flux Reconciliation

```bash
# Watch Flux sync status
flux get kustomizations --watch

# Check HelmRelease status
flux get helmreleases -n fennel-staging

# Monitor pod deployment
kubectl -n fennel-staging get pods -w
```

### 3. Verify Deployment

```bash
# Check if PVC is created with correct storage class
kubectl -n fennel-staging get pvc

# Verify storage class
kubectl -n fennel-staging describe pvc

# Check pod logs
kubectl -n fennel-staging logs -f fennel-staging-node-0
```

## Automated Updates

### Image Tag Updates
When new commits are pushed to fennel-solonet:
1. CI builds new image with tag `sha-NEWCOMMIT`
2. You need to update fennel-prod manually:
   ```bash
   yq eval '.spec.values.image.tag = "sha-NEWCOMMIT"' -i clusters/staging/helmrelease.yaml
   yq eval '.spec.values.node.image.tag = "sha-NEWCOMMIT"' -i clusters/staging/helmrelease.yaml
   ```

### Chart Version Updates
When chart structure changes:
1. New chart version is published (e.g., 0.2.0)
2. Update HelmRelease:
   ```bash
   yq eval '.spec.chart.spec.version = "0.2.0"' -i clusters/staging/helmrelease.yaml
   ```

## Troubleshooting

### Chart Not Found
```bash
# Verify chart is published
curl https://corruptedaesthetic.github.io/fennel-solonet/index.yaml

# Force Flux to refresh
flux reconcile source helm fennel-charts
```

### Storage Class Issues
```bash
# Verify storage class exists
kubectl get storageclass

# Check PVC events
kubectl -n fennel-staging describe pvc
```

### Image Pull Errors
```bash
# Verify image exists
docker pull ghcr.io/corruptedaesthetic/fennel-solonet:sha-XXXXXXX

# Check image pull secrets
kubectl -n fennel-staging get secrets
```

## Best Practices

1. **Version Pinning**: Always pin chart versions in HelmRelease
2. **Testing**: Test chart upgrades in staging before production
3. **Monitoring**: Set up alerts for Flux reconciliation failures
4. **Documentation**: Keep this guide updated with any changes

## Next Steps

1. Set up automated image tag updates using Flux Image Automation
2. Implement production environment with proper key management
3. Add monitoring and alerting for the staging environment
4. Consider using Renovate or similar tools for dependency updates 