#!/bin/bash

# K3s Setup Script for WSL2
# This script installs and configures K3s for Fennel deployment

set -e

echo "=== K3s Installation for Fennel ==="
echo ""

# Install K3s
echo "Installing K3s..."
curl -sfL https://get.k3s.io | sh -

# Wait for K3s to be ready
echo ""
echo "Waiting for K3s to start..."
sleep 10

# Check if K3s is running
if sudo k3s kubectl get nodes > /dev/null 2>&1; then
    echo "✓ K3s is running"
else
    echo "✗ K3s failed to start"
    exit 1
fi

# Create kubectl config for regular user
echo ""
echo "Setting up kubectl access..."
mkdir -p ~/.kube
sudo k3s kubectl config view --raw > ~/.kube/config
chmod 600 ~/.kube/config

# Verify kubectl works
if kubectl get nodes > /dev/null 2>&1; then
    echo "✓ kubectl configured successfully"
else
    echo "✗ kubectl configuration failed"
    exit 1
fi

# Show cluster info
echo ""
echo "=== K3s Cluster Info ==="
kubectl cluster-info
kubectl get nodes

echo ""
echo "=== K3s Setup Complete! ==="
echo ""
echo "You can now deploy Fennel by running:"
echo "  cd /home/neurosx/WORKING_WORKSPACE/fennel-solonet-2503UPGRADE/kubernetes"
echo "  ./deploy-fennel.sh"
echo ""
echo "To uninstall K3s later, run:"
echo "  /usr/local/bin/k3s-uninstall.sh" 