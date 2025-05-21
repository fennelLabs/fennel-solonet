#!/bin/bash

set -e

# Add Parity Helm chart repositories
echo "Adding Helm repositories..."
helm repo add parity https://paritytech.github.io/helm-charts/
helm repo update

# Create namespace if it doesn't exist
kubectl create namespace fennel-blockchain --dry-run=client -o yaml | kubectl apply -f -

# Apply secrets
echo "Applying secrets..."
kubectl apply -f secrets.yaml -n fennel-blockchain

# Use polkadot-node-key-configurator to generate and store node keys
echo "Configuring node keys..."
kubectl apply -f - <<EOF
apiVersion: batch/v1
kind: Job
metadata:
  name: node-key-setup
  namespace: fennel-blockchain
spec:
  template:
    spec:
      containers:
      - name: node-key-configurator
        image: paritytech/polkadot-node-key-configurator:latest
        env:
        - name: SECRET_NAME
          value: "fennel-node-keys"
        - name: NODE_KEY_FIELDS
          value: "chain-node-key peer-node-key peer2-node-key"
      restartPolicy: Never
  backoffLimit: 1
EOF

# Wait for the job to complete
echo "Waiting for node key configuration job to complete..."
kubectl wait --for=condition=complete --timeout=30s job/node-key-setup -n fennel-blockchain

# Deploy the chain node
echo "Deploying chain node..."
helm install chain-node parity/node \
  -f values-chain.yaml \
  --namespace fennel-blockchain

# Wait for the chain node to be ready
echo "Waiting for chain node to be ready..."
kubectl rollout status deployment/chain-node-node -n fennel-blockchain

# Deploy peer nodes
echo "Deploying peer nodes..."
helm install peer-node parity/node \
  -f values-peer.yaml \
  --namespace fennel-blockchain

# Update values-peer2.yaml to use peer2 configurations
sed 's/peer-node/peer2-node/g; s/peer1suri/peer2suri/g; s/30334/30335/g; s/9945/9946/g; s/peer-node-key/peer2-node-key/g' values-peer.yaml > values-peer2.yaml

helm install peer2-node parity/node \
  -f values-peer2.yaml \
  --namespace fennel-blockchain

echo "Blockchain network deployment complete!"
echo "You can check the status with: kubectl get pods -n fennel-blockchain" 