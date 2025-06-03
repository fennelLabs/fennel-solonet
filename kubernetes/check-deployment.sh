#!/bin/bash

# Fennel Deployment Pre-flight Check Script
# This script verifies prerequisites before deployment

set -e

echo "=== Fennel Deployment Pre-flight Check ==="
echo ""

# Configuration
IMAGE_REPO="ghcr.io/corruptedaesthetic/uptodatefennelnetmp"
IMAGE_TAG="${IMAGE_TAG:-sha-204fa8e5891442d07ab060fb2ff7301703b5a4df}"
CHAINSPEC_FILE="../fennelSpec.json"

# Check Docker image availability
echo "1. Checking Docker image availability..."
if docker manifest inspect "${IMAGE_REPO}:${IMAGE_TAG}" > /dev/null 2>&1; then
    echo "   ✓ Image ${IMAGE_REPO}:${IMAGE_TAG} is available"
else
    echo "   ✗ Image ${IMAGE_REPO}:${IMAGE_TAG} not found"
    echo ""
    echo "   Please ensure:"
    echo "   - The GitHub Actions workflow has successfully built and pushed the image"
    echo "   - The repository packages are set to public (or you're authenticated)"
    echo ""
    echo "   To authenticate with GitHub Container Registry:"
    echo "   docker login ghcr.io -u YOUR_GITHUB_USERNAME"
    echo ""
    echo "   Check available packages at:"
    echo "   https://github.com/CorruptedAesthetic/uptodatefennelnetmp/pkgs/container/uptodatefennelnetmp"
    exit 1
fi

# Check chainspec file
echo ""
echo "2. Checking chainspec file..."
if [ -f "$CHAINSPEC_FILE" ]; then
    echo "   ✓ Chainspec file found at $CHAINSPEC_FILE"
    echo "   File size: $(ls -lh $CHAINSPEC_FILE | awk '{print $5}')"
else
    echo "   ✗ Chainspec file not found at $CHAINSPEC_FILE"
    echo "   Please ensure fennelSpec.json exists in the parent directory"
    exit 1
fi

# Check Kubernetes connectivity
echo ""
echo "3. Checking Kubernetes cluster connectivity..."
if kubectl cluster-info > /dev/null 2>&1; then
    echo "   ✓ Connected to Kubernetes cluster"
    kubectl version --short --client
else
    echo "   ✗ Cannot connect to Kubernetes cluster"
    echo "   Please ensure kubectl is configured correctly"
    exit 1
fi

# Check Helm installation
echo ""
echo "4. Checking Helm installation..."
if command -v helm > /dev/null 2>&1; then
    echo "   ✓ Helm is installed"
    helm version --short
else
    echo "   ✗ Helm is not installed"
    echo "   Please install Helm: https://helm.sh/docs/intro/install/"
    exit 1
fi

# Check if Parity Helm repo is added
echo ""
echo "5. Checking Parity Helm repository..."
if helm repo list | grep -q "https://paritytech.github.io/helm-charts/"; then
    echo "   ✓ Parity Helm repository is already added"
else
    echo "   ℹ Parity Helm repository not found, will be added during deployment"
fi

# Summary
echo ""
echo "=== Pre-flight Check Complete ==="
echo ""
echo "All prerequisites are met! You can now run:"
echo "  ./deploy-fennel.sh"
echo ""
echo "Image to be deployed: ${IMAGE_REPO}:${IMAGE_TAG}" 