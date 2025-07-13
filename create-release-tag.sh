#!/bin/bash

# Script to create signed release tags for fennel-solonet
# Usage: ./create-release-tag.sh <version> [message]
# Example: ./create-release-tag.sh 0.4.3 "Enhanced CI/CD with security features"

set -euo pipefail

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

# Check arguments
if [ $# -lt 1 ]; then
    print_error "Usage: $0 <version> [message] [commit]"
    echo "Example: $0 0.4.3 'Enhanced CI/CD with security features'"
    echo "         $0 0.4.3 'Release 0.4.3' abc123def  # tag specific commit"
    exit 1
fi

VERSION="$1"
MESSAGE="${2:-Fennel node v$VERSION}"
TARGET_COMMIT="${3:-HEAD}"  # Default to HEAD if no commit specified

# Validate version format
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "Invalid version format. Expected: X.Y.Z (e.g., 0.4.3)"
    exit 1
fi

TAG_NAME="fennel-node-$VERSION"

print_status "Creating signed release tag: $TAG_NAME"

# Check if tag already exists
if git tag -l | grep -q "^$TAG_NAME$"; then
    print_error "Tag $TAG_NAME already exists"
    exit 1
fi

# Check if we have uncommitted changes
if ! git diff --quiet || ! git diff --staged --quiet; then
    print_warning "You have uncommitted changes. Consider committing them first."
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Show current commit information
CURRENT_COMMIT=$(git rev-parse $TARGET_COMMIT)
if [ "$TARGET_COMMIT" = "HEAD" ]; then
    print_status "Target commit (HEAD): $CURRENT_COMMIT"
else
    print_status "Target commit: $CURRENT_COMMIT"
fi
print_status "Commit details:"
git show --no-patch --format="  %H - %s (%an, %ar)" $TARGET_COMMIT

echo ""
print_status "This tag will point to the above commit."
read -p "Proceed with tagging this commit? (Y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Nn]$ ]]; then
    print_status "Tagging cancelled."
    exit 0
fi

# Check GPG setup
print_status "Checking GPG configuration..."
if ! git config user.signingkey &>/dev/null; then
    print_error "No GPG signing key configured. Run: git config user.signingkey YOUR_KEY_ID"
    exit 1
fi

SIGNING_KEY=$(git config user.signingkey)
print_success "Using GPG key: $SIGNING_KEY"

# Create the signed tag
print_status "Creating signed tag..."
if git tag -s "$TAG_NAME" -m "$MESSAGE" "$TARGET_COMMIT"; then
    print_success "Signed tag created: $TAG_NAME"
else
    print_error "Failed to create signed tag"
    exit 1
fi

# Verify the tag signature
print_status "Verifying tag signature..."
if git verify-tag "$TAG_NAME"; then
    print_success "Tag signature verified"
else
    print_error "Tag signature verification failed"
    exit 1
fi

# Show tag info
print_status "Tag information:"
git show --no-patch --format="  Commit: %H%n  Author: %an <%ae>%n  Date: %ai%n  Message: %s" "$TAG_NAME"

echo ""
print_success "Release tag $TAG_NAME created successfully!"
echo ""
echo "🚀 Next steps:"
echo "   1. Push the tag: git push origin $TAG_NAME"
echo "   2. Watch GitHub Actions: https://github.com/CorruptedAesthetic/fennel-solonet/actions"
echo "   3. Verify the release: https://github.com/CorruptedAesthetic/fennel-solonet/releases"
echo ""
echo "📋 The CI pipeline will:"
echo "   • Verify the tag signature"
echo "   • Build and push Docker image with digest"
echo "   • Generate chainspecs with SHA-256 verification"
echo "   • Package and publish Helm chart"
echo "   • Create GitHub release with all artifacts"

# Ask if user wants to push the tag immediately
echo ""
print_status "Would you like to push the tag now to trigger the production release?"
read -p "Push tag $TAG_NAME to origin? (Y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Nn]$ ]]; then
    print_status "Tag push cancelled. You can push manually later with:"
    echo "   git push origin $TAG_NAME"
else
    print_status "Pushing tag to origin..."
    if git push origin "$TAG_NAME"; then
        print_success "Tag pushed successfully!"
        echo ""
        print_success "🚀 Production release workflow triggered!"
        echo "📊 Watch the build: https://github.com/CorruptedAesthetic/fennel-solonet/actions"
        echo "📦 Release will be available at: https://github.com/CorruptedAesthetic/fennel-solonet/releases"
    else
        print_error "Failed to push tag. You can try manually:"
        echo "   git push origin $TAG_NAME"
    fi
fi
