# Local Kubernetes Development Guide for Fennel

This guide helps you set up a local Kubernetes cluster for testing your Fennel solochain deployment.

## Option 1: Docker Desktop (Easiest)

### Installation
1. Install [Docker Desktop](https://www.docker.com/products/docker-desktop/)
2. Enable Kubernetes in Docker Desktop:
   - Open Docker Desktop settings
   - Go to Kubernetes tab
   - Check "Enable Kubernetes"
   - Click "Apply & Restart"

### Verify Installation
```bash
kubectl cluster-info
kubectl get nodes
```

## Option 2: Minikube (More Flexible)

### Installation
```bash
# macOS
brew install minikube

# Linux
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64
sudo install minikube-linux-amd64 /usr/local/bin/minikube

# Windows (using chocolatey)
choco install minikube
```

### Start Minikube
```bash
# Start with sufficient resources for blockchain nodes
minikube start --cpus=4 --memory=8192 --disk-size=50g

# Enable ingress addon (optional)
minikube addons enable ingress

# Enable metrics-server (for monitoring)
minikube addons enable metrics-server
```

### Access Minikube Dashboard
```bash
minikube dashboard
```

## Option 3: Kind (Kubernetes in Docker)

### Installation
```bash
# macOS/Linux
curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.20.0/kind-linux-amd64
chmod +x ./kind
sudo mv ./kind /usr/local/bin/kind

# Windows (using chocolatey)
choco install kind
```

### Create Cluster
```bash
# Create cluster with custom configuration
cat <<EOF | kind create cluster --config=-
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
  extraPortMappings:
  - containerPort: 30333
    hostPort: 30333
    protocol: TCP
  - containerPort: 30334
    hostPort: 30334
    protocol: TCP
  - containerPort: 30335
    hostPort: 30335
    protocol: TCP
EOF
```

## Option 4: K3s (Lightweight Kubernetes)

### Installation (Linux/WSL2)
```bash
curl -sfL https://get.k3s.io | sh -

# Check status
sudo k3s kubectl get nodes
```

## Installing Helm

### macOS
```bash
brew install helm
```

### Linux
```bash
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
```

### Windows
```bash
choco install kubernetes-helm
```

### Verify Helm Installation
```bash
helm version
```

## Quick Start Commands

Once you have a local cluster running:

```bash
# Check cluster is ready
kubectl cluster-info
kubectl get nodes

# Create namespace for testing
kubectl create namespace fennel-test

# Deploy Fennel (from the kubernetes directory)
cd kubernetes
./deploy-fennel.sh
```

## Useful Commands for Local Development

### Port Forwarding
```bash
# Access RPC endpoint locally
kubectl port-forward -n fennel svc/fennel-solochain-node 9944:9944

# Access Prometheus metrics
kubectl port-forward -n fennel fennel-solochain-node-0 9615:9615
```

### Resource Management
```bash
# Check resource usage
kubectl top nodes
kubectl top pods -n fennel

# Scale validators
kubectl scale statefulset fennel-solochain-node -n fennel --replicas=5
```

### Debugging
```bash
# Get pod details
kubectl describe pod -n fennel fennel-solochain-node-0

# Check events
kubectl get events -n fennel --sort-by='.lastTimestamp'

# Execute commands in pod
kubectl exec -it -n fennel fennel-solochain-node-0 -- /bin/bash
```

## Storage Considerations

For local development, you might want to use local storage:

1. **Docker Desktop**: Uses local volumes automatically
2. **Minikube**: Use `minikube ssh` to access VM storage
3. **Kind**: Mounts local directories as volumes
4. **K3s**: Uses local path provisioner

## Cleanup

### Remove Fennel deployment
```bash
helm uninstall fennel-solochain -n fennel
kubectl delete namespace fennel
```

### Stop/Delete clusters
```bash
# Minikube
minikube stop
minikube delete

# Kind
kind delete cluster

# K3s
/usr/local/bin/k3s-uninstall.sh
```

## Tips for Cursor/VS Code

1. Use the Kubernetes extension to:
   - View cluster resources in the sidebar
   - Apply YAML files directly
   - Stream pod logs
   - Port forward with GUI

2. Terminal integration:
   - Open multiple terminals for different tasks
   - Use split terminals for logs + commands

3. YAML editing:
   - The YAML extension provides schema validation
   - Use snippets for common Kubernetes resources 