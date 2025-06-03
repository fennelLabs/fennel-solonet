# Fennel Solochain Kubernetes Deployment

This directory contains the Kubernetes deployment configuration for the Fennel proof-of-authority solochain using the Parity node Helm chart.

## Prerequisites

- Kubernetes cluster (1.21+)
- kubectl configured to access your cluster
- Helm 3.x installed
- Docker image of your Fennel node pushed to GitHub Container Registry
- fennelSpec.json chainspec file

## Quick Start

1. **Verify prerequisites and image availability**:
   ```bash
   ./check-deployment.sh
   ```
   
   This will verify:
   - Docker image is available at `ghcr.io/corruptedaesthetic/uptodatefennelnetmp`
   - Chainspec file exists
   - Kubernetes cluster is accessible
   - Helm is installed

2. **Deploy the solochain**:
   ```bash
   ./deploy-fennel.sh
   ```

   This script will:
   - Create a namespace called `fennel`
   - Create a Kubernetes secret with your chainspec
   - Deploy 3 validator nodes using the Parity node chart

## Docker Image

The deployment uses the Docker image from the [CorruptedAesthetic/uptodatefennelnetmp](https://github.com/CorruptedAesthetic/uptodatefennelnetmp) repository:
- Image: `ghcr.io/corruptedaesthetic/uptodatefennelnetmp:latest`
- Built automatically via GitHub Actions with srtool

To use a specific version instead of `latest`, update the tag in `fennel-values.yaml`:
```yaml
image:
  repository: ghcr.io/corruptedaesthetic/uptodatefennelnetmp
  tag: YOUR_TAG_HERE  # e.g., main, v1.0.0, or commit SHA
```

## Configuration

### Key Files

- `fennel-values.yaml` - Helm values for customizing the deployment
- `create-chainspec-secret.sh` - Creates a Kubernetes secret from your chainspec
- `deploy-fennel.sh` - Main deployment script
- `check-deployment.sh` - Pre-flight check script to verify prerequisites

### Important Configuration Options

#### Validator Configuration
The deployment is configured for 3 validator nodes by default. Adjust in `fennel-values.yaml`:
```yaml
node:
  replicas: 3  # Number of validators
  role: authority  # Set to 'authority' for validators
```

#### Storage
Each node has persistent volumes for chain data and keystore:
```yaml
node:
  chainData:
    volumeSize: 100Gi  # Adjust based on your needs
  chainKeystore:
    volumeSize: 1Gi
```

#### Network Access
- **RPC/WS Access**: By default, uses ClusterIP. Change to LoadBalancer for external access:
  ```yaml
  node:
    perNodeServices:
      apiService:
        type: LoadBalancer
  ```
- **P2P Communication**: Uses NodePort for peer discovery

## Managing Your Network

### Check Deployment Status
```bash
kubectl get pods -n fennel
kubectl get svc -n fennel
```

### View Logs
```bash
# All nodes
kubectl logs -n fennel -l app.kubernetes.io/instance=fennel-solochain -f

# Specific node
kubectl logs -n fennel fennel-solochain-node-0 -f
```

### Access RPC Endpoint
```bash
# Port forward for local access
kubectl port-forward -n fennel svc/fennel-solochain-node 9944:9944

# Then access at http://localhost:9944
```

### Rotate Session Keys
For each validator node:
```bash
kubectl exec -n fennel fennel-solochain-node-0 -- \
  curl -H 'Content-Type: application/json' \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
  http://localhost:9944
```

### Add/Remove Validators
Since you're using a custom validator pallet with sudo control:
1. Access the RPC endpoint
2. Use sudo extrinsics to add/remove validators through your custom pallet

## Advanced Configuration

### Using External Chainspec URL
Instead of mounting chainspec as a secret, you can download it:
```yaml
node:
  customChainspecUrl: "https://example.com/fennelSpec.json"
```

### Enable Prometheus Monitoring
The deployment includes Prometheus metrics by default on port 9615. To scrape metrics:
```yaml
serviceMonitor:
  enabled: true
```

### Custom Node Keys
For stable peer identities across restarts:
```yaml
node:
  customNodeKey: ["your-node-key-here"]
  # Or generate and persist automatically:
  persistGeneratedNodeKey: true
```

## Troubleshooting

### Image Pull Issues
If pods fail to pull the image:
1. Check image availability: `docker manifest inspect ghcr.io/corruptedaesthetic/uptodatefennelnetmp:latest`
2. Ensure the GitHub package is public or configure image pull secrets
3. Verify the GitHub Actions workflow has successfully built and pushed the image

### Pods Not Starting
- Check chainspec secret exists: `kubectl get secret fennel-chainspec -n fennel`
- Check logs: `kubectl logs -n fennel <pod-name>`
- Verify image can be pulled: `kubectl describe pod -n fennel <pod-name>`

### Peers Not Connecting
- Ensure P2P service is exposed: `kubectl get svc -n fennel`
- Check firewall rules allow NodePort range (30000-32767)
- Verify bootnodes in chainspec are accessible

### Session Keys Issues
- Keys are automatically generated on first start
- To inject existing keys, use the `node.keys` configuration
- Keys are stored in persistent volumes

## Cleanup

To remove the deployment:
```bash
helm uninstall fennel-solochain -n fennel
kubectl delete namespace fennel
```

**Warning**: This will delete all chain data. Back up any important data first!

## Integration with CI/CD

Your GitHub Actions workflow can automatically deploy after building:
```yaml
- name: Deploy to Kubernetes
  run: |
    # Update image tag in values
    sed -i "s/tag: latest/tag: ${{ github.sha }}/" kubernetes/fennel-values.yaml
    # Deploy
    cd kubernetes && ./deploy-fennel.sh
``` 