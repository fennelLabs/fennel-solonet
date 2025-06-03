#!/bin/bash

# Fennel Solochain Kubernetes Deployment Script
# This script deploys the Fennel solochain using Parity's node helm chart

set -e

# Configuration
NAMESPACE=${NAMESPACE:-fennel}
RELEASE_NAME=${RELEASE_NAME:-fennel-solochain}
VALUES_FILE=${VALUES_FILE:-fennel-values.yaml}
HELM_REPO_NAME="parity"
HELM_REPO_URL="https://paritytech.github.io/helm-charts/"

echo "=== Fennel Solochain Kubernetes Deployment ==="
echo ""

# Check prerequisites
echo "Checking prerequisites..."
command -v kubectl >/dev/null 2>&1 || { echo "kubectl is required but not installed. Aborting." >&2; exit 1; }
command -v helm >/dev/null 2>&1 || { echo "helm is required but not installed. Aborting." >&2; exit 1; }

# Create namespace if it doesn't exist
echo "Creating namespace '$NAMESPACE' if it doesn't exist..."
kubectl create namespace $NAMESPACE --dry-run=client -o yaml | kubectl apply -f -

# ✅ POLKADOT STANDARD: Skip chainspec secret creation - using embedded chainspec
echo ""
echo "Using embedded chainspec from Docker image (Polkadot standard approach)..."
echo "Skipping chainspec secret creation - validators will use built-in runtime presets"

# Add Parity helm repository
echo ""
echo "Adding Parity helm repository..."
helm repo add $HELM_REPO_NAME $HELM_REPO_URL
helm repo update

# Deploy the helm chart
echo ""
echo "Deploying Fennel solochain..."
helm upgrade --install $RELEASE_NAME $HELM_REPO_NAME/node \
    --namespace $NAMESPACE \
    --values $VALUES_FILE \
    --wait \
    --timeout 10m

echo ""
echo "=== Deployment Complete ==="
echo ""
echo "✅ Using Polkadot standard approach: embedded runtime presets from GitHub Actions image"
echo "   All validators now use the same genesis configuration via built-in runtime presets"
echo "   Genesis consistency guaranteed across k3s and external validators"
echo ""
echo "To check the status of your deployment:"
echo "  kubectl get pods -n $NAMESPACE"
echo ""
echo "To view logs:"
echo "  kubectl logs -n $NAMESPACE -l app.kubernetes.io/instance=$RELEASE_NAME -f"
echo ""
echo "To access RPC endpoint (port-forward):"
echo "  kubectl port-forward -n $NAMESPACE svc/$RELEASE_NAME-node 9944:9944"
echo ""
echo "To get validator session keys:"
echo "  kubectl exec -n $NAMESPACE $RELEASE_NAME-node-0 -- curl -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"author_rotateKeys\"}' http://localhost:9944" 