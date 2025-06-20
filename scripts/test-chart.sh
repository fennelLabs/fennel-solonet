#!/bin/bash
set -e

echo "Testing fennel-node Helm chart..."

# Change to the chart directory
CHART_DIR="$(dirname "$0")/../Charts/fennel-node"
cd "$CHART_DIR"

# Clean up old dependencies
echo "Cleaning old dependencies..."
rm -rf charts/ Chart.lock

# Update dependencies
echo "Updating dependencies..."
helm dependency update

# Lint the chart
echo "Linting chart with default values..."
helm lint .

echo "Linting chart with staging values..."
helm lint . -f values-staging.yaml

echo "Linting chart with test values..."
helm lint . -f examples/test-values.yaml

# Dry run with different configurations
echo "Testing template rendering with default values..."
helm template test-release . > /dev/null

echo "Testing template rendering with staging values..."
helm template test-release . -f values-staging.yaml > /dev/null

echo "Testing template rendering with test values..."
helm template test-release . -f examples/test-values.yaml > /dev/null

echo "All tests passed! ✅"
echo ""
echo "To install the chart locally, run:"
echo "  helm install fennel-test $CHART_DIR -f $CHART_DIR/examples/test-values.yaml" 