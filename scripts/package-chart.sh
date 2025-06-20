#!/bin/bash
set -e

# Script to package the fennel-node Helm chart
echo "Packaging fennel-node Helm chart..."

# Change to the chart directory
cd "$(dirname "$0")/../Charts/fennel-node"

# Clean up old dependencies
echo "Cleaning up old dependencies..."
rm -rf charts/

# Update dependencies
echo "Updating chart dependencies..."
helm dependency update

# Package the chart
echo "Packaging chart..."
helm package . --destination ../../release/

# List the packaged chart
echo "Chart packaged successfully:"
ls -la ../../release/fennel-node-*.tgz

echo "Done!" 