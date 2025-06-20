June 19 2025 9:38pm
# Fennel Chart Configuration Validation Guide

## Chart Version Validation

### 1. Check if Chart is Published

The chart needs to be published to GitHub Pages before it can be used by Flux:

```bash
# Check if version 0.2.0 is available
curl -s https://corruptedaesthetic.github.io/fennel-solonet/index.yaml | grep -A2 "version: 0.2.0"
```

**Expected output** (when published):
```yaml
  - version: 0.2.0
    created: "2025-01-19T..."
    digest: ...
```

**Current status**: ❌ Not yet published (needs CI/CD to run)

### 2. Publish the Chart

To publish the chart, you need to:

1. **Push changes to fennel-solonet**:
   ```bash
   cd ~/MAINPROJECTFOLDER/fennel-deploy/fennel-solonet
   git add .
   git commit -m "Update to Parity node chart v5.15.0"
   git push origin main
   ```

2. **Wait for CI/CD** to:
   - Build Docker image
   - Package Helm chart v0.2.0
   - Publish to GitHub Pages

3. **Verify publication** (after ~5-10 minutes):
   ```bash
   curl -s https://corruptedaesthetic.github.io/fennel-solonet/index.yaml | grep -A2 "version: 0.2.0"
   ```

## Values Structure Validation

### Confirmed Parity Node Chart v5.15.0 Structure

Based on the helm show values output, the correct path for storage class in Parity node chart v5.15.0 is:

```yaml
node:
  chainData:
    storageClass: ""  # ← This is the correct path
    volumeSize: 100Gi
    pruning: 1000
```

### Our Configuration is Correct ✅

In our `values-staging.yaml` and the new HelmRelease, we're using:

```yaml
node:
  node:
    chainData:
      storageClass: "local-path"  # ✅ Correct path
      volumeSize: "100Gi"
```

The double `node.node` is because:
- First `node:` is our subchart name (Parity node chart)
- Second `node:` is the section within the Parity chart

## Complete Validation Checklist

### Pre-deployment Validation

1. **Local Chart Version**:
   ```bash
   grep "version:" Charts/fennel-node/Chart.yaml
   # Should show: version: 0.2.0
   ```

2. **Parity Dependency Version**:
   ```bash
   grep -A2 "dependencies:" Charts/fennel-node/Chart.yaml
   # Should show: version: "5.15.0"
   ```

3. **Test Chart Locally**:
   ```bash
   cd Charts/fennel-node
   helm dependency update
   helm lint .
   helm template test . -f values-staging.yaml > /tmp/test-output.yaml
   grep storageClassName /tmp/test-output.yaml
   # Should show: storageClassName: local-path
   ```

### Post-deployment Validation

1. **Check Flux Sources**:
   ```bash
   flux get sources helm
   # Should show fennel-charts as Ready
   ```

2. **Check HelmRelease**:
   ```bash
   flux get helmrelease -n fennel-staging
   # Should show fennel-staging with version 0.2.0
   ```

3. **Verify PVC Storage Class**:
   ```bash
   kubectl -n fennel-staging get pvc -o yaml | grep -A1 "storageClassName"
   # Should show: storageClassName: local-path
   ```

4. **Check Pod Status**:
   ```bash
   kubectl -n fennel-staging get pods
   kubectl -n fennel-staging describe pod fennel-staging-node-0
   ```

## Troubleshooting

### If Chart Not Found

1. **Check GitHub Pages status**:
   ```bash
   curl -I https://corruptedaesthetic.github.io/fennel-solonet/index.yaml
   # Should return: HTTP/2 200
   ```

2. **Check chart-releaser action**:
   - Go to fennel-solonet GitHub repo
   - Check Actions tab for publish workflow
   - Verify chart-releaser step succeeded

3. **Force Flux to refresh**:
   ```bash
   flux reconcile source helm fennel-charts -n flux-system
   ```

### If Storage Class Still Wrong

1. **Delete old resources**:
   ```bash
   kubectl -n fennel-staging delete helmrelease fennel-staging
   kubectl -n fennel-staging delete pvc --all
   ```

2. **Re-apply HelmRelease**:
   ```bash
   cd ~/MAINPROJECTFOLDER/\ fennel-prod
   kubectl apply -f clusters/staging/helmrelease.yaml
   ```

3. **Monitor recreation**:
   ```bash
   kubectl -n fennel-staging get events --watch
   ```

## Summary

✅ **Values structure is correct**: We're using `node.node.chainData.storageClass` which maps correctly to the Parity chart structure

❌ **Chart not yet published**: Need to push changes to trigger CI/CD

📋 **Next steps**:
1. Push fennel-solonet changes to trigger CI/CD
2. Wait for chart publication (~5-10 minutes)
3. Flux will automatically pick up the new chart
4. Verify PVC uses `local-path` storage class 