#!/bin/bash

# Test script for validating the enhanced GitHub Actions workflow
# This checks for common issues before committing

set -euo pipefail

echo "🧪 Testing Enhanced GitHub Actions Workflow"
echo "==========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if we're in the right directory
if [ ! -f ".github/workflows/publish.yml" ]; then
    print_error "Not in fennel-solonet root directory"
    exit 1
fi

print_status "Checking workflow syntax..."

# Basic YAML syntax check (if yamllint is available)
if command -v yamllint &> /dev/null; then
    if yamllint .github/workflows/publish.yml; then
        print_success "YAML syntax is valid"
    else
        print_error "YAML syntax errors found"
        exit 1
    fi
else
    print_warning "yamllint not available, skipping YAML syntax check"
fi

# Check for required fields in values.yaml files
print_status "Checking Helm chart values..."

REQUIRED_FIELDS=("image.digest" "image.tag" "releaseTag")
for field in "${REQUIRED_FIELDS[@]}"; do
    if grep -q "$field:" Charts/fennel-node/values.yaml; then
        print_success "Found $field in values.yaml"
    else
        print_error "Missing $field in values.yaml"
        exit 1
    fi
done

# Check staging values
STAGING_FIELDS=("image.digest" "node.customChainspecSha256")
for field in "${STAGING_FIELDS[@]}"; do
    if grep -q "$field:" Charts/fennel-node/values-staging.yaml; then
        print_success "Found $field in values-staging.yaml"
    else
        print_error "Missing $field in values-staging.yaml"
        exit 1
    fi
done

# Check template helper exists
if grep -q "fennel-node.image" Charts/fennel-node/templates/_helpers.tpl; then
    print_success "Image helper template found"
else
    print_error "Missing image helper template"
    exit 1
fi

# Check template uses the helper
if grep -q "include \"fennel-node.image\"" Charts/fennel-node/templates/statefulset.yaml; then
    print_success "StatefulSet uses secure image helper"
else
    print_error "StatefulSet not using secure image helper"
    exit 1
fi

# Check workflow has tag verification
if grep -q "Verify tag signature" .github/workflows/publish.yml; then
    print_success "Tag verification step found"
else
    print_error "Missing tag verification step"
    exit 1
fi

# Check workflow has chainspec SHA computation
if grep -q "Compute chainspec SHA-256" .github/workflows/publish.yml; then
    print_success "Chainspec SHA computation found"
else
    print_error "Missing chainspec SHA computation"
    exit 1
fi

# Check workflow has fail_on_unmatched_files
if grep -q "fail_on_unmatched_files: true" .github/workflows/publish.yml; then
    print_success "Release failure protection enabled"
else
    print_error "Missing release failure protection"
    exit 1
fi

print_success "All checks passed! Enhanced CI/CD is ready."
echo ""
echo "🚀 Next steps:"
echo "   1. Commit and push these changes"
echo "   2. Create a test tag: git tag -s fennel-node-0.5.0-test -m 'Test release'"
echo "   3. Push the tag: git push origin fennel-node-0.5.0-test"
echo "   4. Watch the GitHub Actions workflow"
echo ""
echo "📚 Documentation: docs/ENHANCED-CICD.md"
