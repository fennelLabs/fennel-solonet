#!/bin/bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CHART_PATH="$PROJECT_ROOT/Charts/fennel-node"

echo -e "${GREEN}Testing Fennel Node Helm Chart${NC}"
echo "Chart path: $CHART_PATH"

# Check if Helm is installed
if ! command -v helm &> /dev/null; then
    echo -e "${RED}Helm is not installed. Please install Helm first.${NC}"
    exit 1
fi

# Add Parity Helm repository
echo -e "\n${YELLOW}Adding Parity Helm repository...${NC}"
helm repo add parity https://paritytech.github.io/helm-charts || true
helm repo update

# Update dependencies
echo -e "\n${YELLOW}Updating Helm dependencies...${NC}"
cd "$CHART_PATH"
helm dependency update

# Lint the base chart
echo -e "\n${YELLOW}Linting base chart...${NC}"
if helm lint .; then
    echo -e "${GREEN}✓ Base chart linting passed${NC}"
else
    echo -e "${RED}✗ Base chart linting failed${NC}"
    exit 1
fi

# Lint with staging values
echo -e "\n${YELLOW}Linting with staging values...${NC}"
if helm lint . -f values-staging.yaml; then
    echo -e "${GREEN}✓ Staging values linting passed${NC}"
else
    echo -e "${RED}✗ Staging values linting failed${NC}"
    exit 1
fi

# Test template rendering
echo -e "\n${YELLOW}Testing template rendering...${NC}"
helm template fennel-staging . -f values-staging.yaml > /dev/null
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Template rendering successful${NC}"
else
    echo -e "${RED}✗ Template rendering failed${NC}"
    exit 1
fi

# Package the chart
echo -e "\n${YELLOW}Packaging chart...${NC}"
mkdir -p "$PROJECT_ROOT/release"
helm package . --destination "$PROJECT_ROOT/release"

# List packaged chart
echo -e "\n${GREEN}Chart packaged successfully:${NC}"
ls -la "$PROJECT_ROOT/release"/*.tgz

# Show chart information
echo -e "\n${YELLOW}Chart information:${NC}"
helm show chart "$PROJECT_ROOT/release"/*.tgz

echo -e "\n${GREEN}All tests passed! Chart is ready for CI/CD.${NC}" 