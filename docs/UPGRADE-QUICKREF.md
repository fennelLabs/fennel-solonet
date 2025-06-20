# Quick Reference: Fennel Chart v0.2.0 Upgrade

## What Changed

### Chart Dependencies
- **OLD**: Parity node chart v0.10.0
- **NEW**: Parity node chart v5.15.0

### Storage Class Configuration
- **OLD**: `storageClass: "local-path"` (root level)
- **NEW**: `node.node.chainData.storageClass: "local-path"`

### Key Paths Changed
```yaml
# OLD Structure
storageClass: "local-path"
node:
  chainData:
    volumeSize: "100Gi"
  perNodeServices:
    createClusterIPService: true

# NEW Structure  
node:
  node:
    chainData:
      storageClass: "local-path"
      volumeSize: "100Gi"
    perNodeServices:
      apiService:
        enabled: true
```

## Action Items

### 1. In fennel-solonet (✅ DONE)
- [x] Updated Chart.yaml to use Parity v5.15.0
- [x] Rewrote values.yaml for new structure
- [x] Updated values-staging.yaml
- [x] Created new chart package v0.2.0

### 2. In fennel-prod (TODO)
- [ ] Update HelmRelease chart version: `0.1.3` → `0.2.0`
- [ ] Update HelmRelease values structure (see staging-deployment-guide.md)
- [ ] Commit and push changes
- [ ] Monitor Flux reconciliation

## Testing Commands

```bash
# Test locally before deploying
cd fennel-solonet
./scripts/test-chart.sh

# Package new version
./scripts/package-chart.sh

# In fennel-prod, after updating:
flux reconcile helmrelease fennel-staging -n fennel-staging
kubectl -n fennel-staging get pods -w
```

## Common Issues

1. **PVC stuck pending**: Check storage class name matches
2. **Image pull errors**: Verify image tag format
3. **Chart not found**: Wait for GitHub Pages to update (~5 min) 