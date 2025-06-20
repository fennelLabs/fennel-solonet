#!/bin/bash
set -euo pipefail

# Fennel Local Development Setup with k3d
echo "🚀 Setting up Fennel local development environment with k3d"

# Configuration
CLUSTER_NAME="fennel-local"
K3D_VERSION="v5.6.0"
KUBECTL_VERSION="v1.28.0"
HELM_VERSION="v3.13.0"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running on WSL
if grep -q Microsoft /proc/version 2>/dev/null; then
    log "Detected WSL environment"
    WSL_ENV=true
else
    WSL_ENV=false
fi

# Install k3d if not present
install_k3d() {
    if ! command -v k3d &> /dev/null; then
        log "Installing k3d..."
        curl -s https://raw.githubusercontent.com/rancher/k3d/main/install.sh | bash
    else
        log "k3d already installed: $(k3d version)"
    fi
}

# Install kubectl if not present
install_kubectl() {
    if ! command -v kubectl &> /dev/null; then
        log "Installing kubectl..."
        curl -LO "https://dl.k8s.io/release/${KUBECTL_VERSION}/bin/linux/amd64/kubectl"
        chmod +x kubectl
        sudo mv kubectl /usr/local/bin/
    else
        log "kubectl already installed: $(kubectl version --client --short 2>/dev/null || echo 'kubectl present')"
    fi
}

# Install helm if not present
install_helm() {
    if ! command -v helm &> /dev/null; then
        log "Installing Helm..."
        curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
    else
        log "Helm already installed: $(helm version --short)"
    fi
}

# Create k3d cluster with proper configuration
create_cluster() {
    if k3d cluster list | grep -q "${CLUSTER_NAME}"; then
        warn "Cluster ${CLUSTER_NAME} already exists. Deleting first..."
        k3d cluster delete "${CLUSTER_NAME}"
    fi
    
    log "Creating k3d cluster: ${CLUSTER_NAME}"
    
    # Create cluster with:
    # - Port forwarding for node services
    # - Persistent volume support
    # - Registry for local images
    k3d cluster create "${CLUSTER_NAME}" \
        --port "30333:30333@server:0" \
        --port "9944:9944@server:0" \
        --port "9933:9933@server:0" \
        --port "9615:9615@server:0" \
        --k3s-arg "--disable=traefik@server:0" \
        --registry-create "fennel-registry:5000" \
        --volume "$(pwd)/data:/data@server:0" \
        --wait
    
    log "Cluster created successfully!"
    
    # Set kubectl context
    kubectl config use-context "k3d-${CLUSTER_NAME}"
}

# Install required Kubernetes components
install_k8s_components() {
    log "Installing Kubernetes components..."
    
    # Install Flux CLI if not present
    if ! command -v flux &> /dev/null; then
        log "Installing Flux CLI..."
        curl -s https://fluxcd.io/install.sh | bash
        export PATH="${HOME}/.flux/bin:${PATH}"
    fi
    
    # Create namespace
    kubectl create namespace fennel-local --dry-run=client -o yaml | kubectl apply -f -
    
    # Add Parity Helm repository
    log "Adding Parity Helm repository..."
    helm repo add parity https://paritytech.github.io/helm-charts
    helm repo update
    
    # Create storage class for local development
    log "Creating local storage class..."
    cat <<EOF | kubectl apply -f -
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: local-path
  annotations:
    storageclass.kubernetes.io/is-default-class: "true"
provisioner: rancher.io/local-path
volumeBindingMode: WaitForFirstConsumer
reclaimPolicy: Delete
EOF
}

# Create local development secrets
create_secrets() {
    log "Creating development secrets..."
    
    # Create dummy validator keys for Alice
    kubectl create secret generic validator-keys \
        --namespace=fennel-local \
        --from-literal=alice-gran="0x88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee" \
        --from-literal=alice-babe="0x416c696365" \
        --dry-run=client -o yaml | kubectl apply -f -
    
    # Create dummy node key
    kubectl create secret generic node-key \
        --namespace=fennel-local \
        --from-literal=node.key="0000000000000000000000000000000000000000000000000000000000000001" \
        --dry-run=client -o yaml | kubectl apply -f -
    
    log "Development secrets created"
}

# Create development chainspec ConfigMap
create_chainspec_configmap() {
    log "Creating chainspec ConfigMap..."
    
    # Check if development chainspec exists
    if [ ! -f "chainspecs/development/development.json" ]; then
        error "Development chainspec not found! Please run GitHub Actions build first or create manually."
        return 1
    fi
    
    # Create ConfigMap from chainspec
    kubectl create configmap fennel-chainspec \
        --namespace=fennel-local \
        --from-file=development.json=chainspecs/development/development.json \
        --dry-run=client -o yaml | kubectl apply -f -
    
    log "Chainspec ConfigMap created"
}

# Build and load local image
build_and_load_image() {
    log "Building and loading Fennel node image..."
    
    # Build the image locally
    docker build -t fennel-node:local .
    
    # Import image into k3d cluster
    k3d image import fennel-node:local --cluster="${CLUSTER_NAME}"
    
    log "Image built and loaded into cluster"
}

# Create development values file for Helm
create_dev_values() {
    log "Creating development values file..."
    
    cat > values-local.yaml <<EOF
# Local development values for fennel-node chart
image:
  repository: "fennel-node"
  tag: "local"
  pullPolicy: Never  # Use local image

chainspec:
  file: "development.json"

node:
  enabled: true
  
  image:
    repository: "fennel-node"
    tag: "local"
    pullPolicy: Never
  
  node:
    chain: ""
    command: "fennel-node"
    replicas: 1
    role: "authority"
    
    customChainspec: true
    customChainspecPath: "/chainspec/development.json"
    
    chainData:
      storageClass: "local-path"
      volumeSize: "20Gi"  # Smaller for local dev
      pruning: 256        # Aggressive pruning for local
      
    chainKeystore:
      storageClass: "local-path"
      volumeSize: "1Gi"
      
    # Development flags
    flags:
      - "--dev"
      - "--tmp"  # Use temporary storage
      - "--prometheus-external"
      - "--rpc-external"
      - "--ws-external"
      - "--rpc-cors=all"
      - "--rpc-methods=unsafe"
      - "--log=info,runtime::system=debug"
      - "--enable-offchain-indexing=true"
      
    perNodeServices:
      apiService:
        enabled: true
        type: NodePort
        nodePort: 9944
      p2pService:
        enabled: true
        type: NodePort
        nodePort: 30333
        
    prometheus:
      enabled: true
      port: 9615

  # Pod configuration for development
  resources:
    requests:
      cpu: 200m
      memory: 512Mi
    limits:
      cpu: 1
      memory: 2Gi

  # Service monitor
  serviceMonitor:
    enabled: false  # Disable for local dev
    
  extraVolumes:
    - name: chainspec
      configMap:
        name: "fennel-chainspec"
        
  extraVolumeMounts:
    - name: chainspec
      mountPath: /chainspec
      readOnly: true
EOF

    log "Development values file created: values-local.yaml"
}

# Deploy Fennel node
deploy_fennel() {
    log "Deploying Fennel node..."
    
    # Update Helm dependencies
    cd Charts/fennel-node
    helm dependency update
    cd ../..
    
    # Install/upgrade the chart
    helm upgrade --install fennel-local \
        ./Charts/fennel-node \
        --namespace=fennel-local \
        --values=values-local.yaml \
        --wait \
        --timeout=10m
    
    log "Fennel node deployed successfully!"
}

# Show connection information
show_connection_info() {
    log "🎉 Local Fennel development environment is ready!"
    echo
    echo "Connection Information:"
    echo "======================"
    echo "• RPC Endpoint:  http://localhost:9944"
    echo "• WS Endpoint:   ws://localhost:9944"
    echo "• P2P Port:      30333"
    echo "• Metrics:       http://localhost:9615/metrics"
    echo
    echo "Useful Commands:"
    echo "==============="
    echo "• View pods:     kubectl get pods -n fennel-local"
    echo "• View logs:     kubectl logs -n fennel-local -l app.kubernetes.io/name=node -f"
    echo "• Port forward:  kubectl port-forward -n fennel-local svc/fennel-local-node 9944:9944"
    echo "• Delete cluster: k3d cluster delete ${CLUSTER_NAME}"
    echo
    echo "Test the node:"
    echo "============="
    echo "curl -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\", \"params\":[]}' http://localhost:9944"
}

# Cleanup function
cleanup() {
    log "Cleaning up on exit..."
    jobs -p | xargs -r kill 2>/dev/null || true
}

# Main execution
main() {
    trap cleanup EXIT
    
    log "Starting Fennel local development setup..."
    
    # Check if we're in the right directory
    if [ ! -f "Cargo.toml" ] || [ ! -d "node" ]; then
        error "Please run this script from the fennel-solonet root directory"
        exit 1
    fi
    
    # Install required tools
    install_k3d
    install_kubectl
    install_helm
    
    # Create and configure cluster
    create_cluster
    install_k8s_components
    
    # Build and prepare application
    build_and_load_image
    create_secrets
    
    # Only create chainspec if it doesn't exist
    if [ -f "chainspecs/development/development.json" ]; then
        create_chainspec_configmap
    else
        warn "Development chainspec not found. Creating minimal one..."
        mkdir -p chainspecs/development
        echo '{"name":"Fennel Development","id":"fennel_dev","chainType":"Local","bootNodes":[],"protocolId":"fennel"}' > chainspecs/development/development.json
        create_chainspec_configmap
    fi
    
    # Deploy application
    create_dev_values
    deploy_fennel
    
    # Show connection info
    show_connection_info
}

# Handle script arguments
case "${1:-setup}" in
    "setup")
        main
        ;;
    "cleanup")
        log "Cleaning up k3d cluster..."
        k3d cluster delete "${CLUSTER_NAME}" 2>/dev/null || true
        docker rmi fennel-node:local 2>/dev/null || true
        rm -f values-local.yaml
        log "Cleanup complete"
        ;;
    "rebuild")
        log "Rebuilding and redeploying..."
        build_and_load_image
        helm upgrade --install fennel-local \
            ./Charts/fennel-node \
            --namespace=fennel-local \
            --values=values-local.yaml \
            --wait
        log "Rebuild complete"
        ;;
    *)
        echo "Usage: $0 [setup|cleanup|rebuild]"
        echo "  setup   - Create k3d cluster and deploy Fennel (default)"
        echo "  cleanup - Remove k3d cluster and clean up"
        echo "  rebuild - Rebuild image and redeploy"
        exit 1
        ;;
esac 