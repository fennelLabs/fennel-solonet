#!/bin/bash
set -e

echo "🔍 Validating fennel-node Helm chart..."

# Change to chart directory
CHART_DIR="$(dirname "$0")/../Charts/fennel-node"
cd "$CHART_DIR"

echo ""
echo "📋 Chart Information:"
echo "===================="
grep "^version:" Chart.yaml || true
grep "version:" Chart.yaml | grep -v "^version:" | head -1 || true

echo ""
echo "🔄 Updating dependencies..."
helm dependency update

echo ""
echo "✅ Linting chart..."
helm lint . -f values-staging.yaml

echo ""
echo "🧪 Testing template rendering..."
if helm template test . -f values-staging.yaml > /tmp/fennel-test-output.yaml 2>&1; then
    echo "✅ Template rendering successful!"
    
    echo ""
    echo "🔍 Verifying storage class configuration:"
    echo "========================================"
    grep -B2 -A2 "storageClassName" /tmp/fennel-test-output.yaml | grep -E "(name:|storageClassName:)" || true
    
    STORAGE_CLASS_COUNT=$(grep -c "storageClassName: local-path" /tmp/fennel-test-output.yaml || true)
    echo ""
    echo "✅ Found $STORAGE_CLASS_COUNT PVCs with 'local-path' storage class"
else
    echo "❌ Template rendering failed!"
    cat /tmp/fennel-test-output.yaml
    exit 1
fi

echo ""
echo "📦 Chart validation complete!"
echo ""
echo "Next steps:"
echo "1. Push changes to trigger CI/CD"
echo "2. Wait for chart to be published to GitHub Pages"
echo "3. Check: curl -s https://corruptedaesthetic.github.io/fennel-solonet/index.yaml | grep -A2 'version: 0.2.0'" 