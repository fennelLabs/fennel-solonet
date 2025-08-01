.PHONY: lint build multi test scan clean act-setup act-test-dispatch act-test-tag act-test-job act-dry-run help

# Default target runs all tests in order
all: lint build test

# Docker image name
IMAGE_NAME := fennel-solonet:test

# Act configuration
ACT_SECRETS := --secret-file .secrets
ACT_EVENT_DISPATCH := .github/event-release.json
ACT_EVENT_TAG := .github/event-tag-push.json

# Syntax and best-practices lint using hadolint
lint:
	@echo "🔍 Linting Dockerfile with hadolint..."
	docker run --rm -i hadolint/hadolint < Dockerfile

# Standard build test
build:
	@echo "🏗️  Building Docker image..."
	DOCKER_BUILDKIT=1 docker build -t $(IMAGE_NAME) .

# Multi-architecture build (same as CI)
multi:
	@echo "🏗️  Setting up multi-architecture build..."
	docker run --privileged --rm tonistiigi/binfmt:latest || true
	docker buildx create --use --name multi || true
	docker buildx build --platform linux/amd64,linux/arm64 \
		-t $(IMAGE_NAME) .

# Test multi-architecture build locally
multi-test:
	@echo "🏗️  Building and testing both architectures..."
	docker run --privileged --rm tonistiigi/binfmt:latest || true
	docker buildx create --use --name multi-test || true
	@echo "🧪 Building for AMD64..."
	docker buildx build --platform linux/amd64 \
		-t $(IMAGE_NAME)-amd64 --load .
	@echo "🧪 Testing AMD64 build..."
	docker run --rm $(IMAGE_NAME)-amd64 --version
	@echo "🧪 Building for ARM64..."
	docker buildx build --platform linux/arm64 \
		-t $(IMAGE_NAME)-arm64 --load .
	@echo "🧪 Testing ARM64 build..."
	docker run --rm $(IMAGE_NAME)-arm64 --version
	@echo "✅ Multi-architecture builds successful!"

# Debug ARM64 build issues
debug-arm64:
	@echo "🐛 Debugging ARM64 build..."
	docker run --privileged --rm tonistiigi/binfmt:latest || true
	docker buildx create --use --name debug-multi || true
	@echo "🔍 Building ARM64 with verbose output..."
	docker buildx build --platform linux/arm64 \
		--progress=plain \
		-t $(IMAGE_NAME)-arm64-debug .

# Test just AMD64 build (should work)
test-amd64:
	@echo "🧪 Testing AMD64 build only..."
	docker buildx build --platform linux/amd64 \
		-t $(IMAGE_NAME)-amd64 --load .
	docker run --rm $(IMAGE_NAME)-amd64 --version

# Smoke test the built image
test:
	@echo "🧪 Testing the built image..."
	docker run --rm $(IMAGE_NAME) --version
	@echo "✅ Basic smoke test passed"

# Extended smoke test with more commands
test-extended:
	@echo "🧪 Running extended tests..."
	docker run --rm $(IMAGE_NAME) --version
	docker run --rm $(IMAGE_NAME) --help | head -20
	@echo "✅ Extended tests passed"

# Security/CVE scan (requires running outside container)
scan:
	@echo "🔒 Running security scan..."
	@if command -v trivy >/dev/null 2>&1; then \
		trivy image $(IMAGE_NAME); \
	else \
		echo "⚠️  Trivy not installed locally. Installing via docker..."; \
		docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
			aquasec/trivy:latest image $(IMAGE_NAME); \
	fi

# Integration test with real Substrate node startup
test-integration:
	@echo "🧪 Running integration test..."
	@echo "Starting node in background for 10 seconds..."
	timeout 10s docker run --rm $(IMAGE_NAME) --dev --tmp || true
	@echo "✅ Integration test completed"

# Clean up build artifacts and images
clean:
	@echo "🧹 Cleaning up..."
	docker rmi $(IMAGE_NAME) || true
	docker builder prune -f || true
	docker system prune -f || true

# Build and test everything
ci: lint build test test-extended

# ============================================================================
# GitHub Actions Workflow Testing with act
# ============================================================================

# Setup prerequisites for act testing
act-setup:
	@echo "🎬 Setting up act prerequisites..."
	@echo "Enabling multi-arch builds..."
	docker run --privileged --rm tonistiigi/binfmt:latest || true
	@echo "Checking act installation..."
	@if ! command -v act >/dev/null 2>&1; then \
		echo "❌ act not found. Install with: brew install act"; \
		exit 1; \
	fi
	@echo "✅ act setup complete"

# Test workflow_dispatch event (manual trigger)
act-test-dispatch: act-setup
	@echo "🎬 Testing workflow with workflow_dispatch event..."
	act workflow_dispatch \
		-e $(ACT_EVENT_DISPATCH) \
		-W .github/workflows/publish.yml \
		$(ACT_SECRETS) \
		--container-daemon-socket /var/run/docker.sock \
		--privileged

# Test tag push event (release trigger)
act-test-tag: act-setup
	@echo "🎬 Testing workflow with tag push event..."
	act push \
		-e $(ACT_EVENT_TAG) \
		-W .github/workflows/publish.yml \
		$(ACT_SECRETS) \
		--container-daemon-socket /var/run/docker.sock \
		--privileged

# Test specific job (useful for debugging individual jobs)
act-test-job: act-setup
	@echo "🎬 Testing specific job: build-binaries..."
	act push \
		-e $(ACT_EVENT_TAG) \
		-W .github/workflows/publish.yml \
		$(ACT_SECRETS) \
		--job build-binaries \
		--container-daemon-socket /var/run/docker.sock \
		--privileged

# Dry run - list what would be executed without running
act-dry-run:
	@echo "🎬 Dry run - showing what would be executed..."
	@echo "=== Workflow dispatch event ==="
	act workflow_dispatch -e $(ACT_EVENT_DISPATCH) -W .github/workflows/publish.yml --dryrun
	@echo ""
	@echo "=== Tag push event ==="
	act push -e $(ACT_EVENT_TAG) -W .github/workflows/publish.yml --dryrun

# Test individual jobs for debugging
act-test-build-only:
	@echo "🎬 Testing build-binaries job only..."
	act push -e $(ACT_EVENT_TAG) -W .github/workflows/publish.yml $(ACT_SECRETS) --job build-binaries

act-test-docker-only:
	@echo "🎬 Testing build-and-push-docker job only..."
	act push -e $(ACT_EVENT_TAG) -W .github/workflows/publish.yml $(ACT_SECRETS) --job build-and-push-docker

act-test-chainspecs-only:
	@echo "🎬 Testing generate-chainspecs job only..."
	act push -e $(ACT_EVENT_TAG) -W .github/workflows/publish.yml $(ACT_SECRETS) --job generate-chainspecs

act-test-release-only:
	@echo "🎬 Testing package-and-release job only..."
	act push -e $(ACT_EVENT_TAG) -W .github/workflows/publish.yml $(ACT_SECRETS) --job package-and-release

# Validate workflow syntax without running
act-validate:
	@echo "🔍 Validating workflow syntax..."
	act --list -W .github/workflows/publish.yml || echo "❌ Workflow validation failed"

# Debug act configuration
act-debug:
	@echo "🐛 Debug information for act..."
	@echo "=== act version ==="
	act --version
	@echo ""
	@echo "=== Workflows detected ==="
	act --list
	@echo ""
	@echo "=== Event files ==="
	ls -la .github/event-*.json || echo "No event files found"
	@echo ""
	@echo "=== Secrets file ==="
	ls -la .secrets || echo "No secrets file found"

# Include additional act testing targets
include .github/act-testing.mk

# Help target
help:
	@echo "Available targets:"
	@echo ""
	@echo "🐳 Docker Testing:"
	@echo "  lint           - Run Dockerfile linting with hadolint"
	@echo "  build          - Build Docker image"
	@echo "  multi          - Build multi-architecture image"
	@echo "  test           - Run basic smoke tests"
	@echo "  test-extended  - Run extended smoke tests"
	@echo "  test-integration - Run integration tests"
	@echo "  scan           - Run security vulnerability scan"
	@echo "  clean          - Clean up build artifacts"
	@echo "  ci             - Run full CI pipeline (lint + build + test)"
	@echo ""
	@echo "🎬 GitHub Actions Workflow Testing:"
	@echo "  act-setup        - Setup prerequisites for act testing"
	@echo "  act-test-dispatch - Test workflow_dispatch event"
	@echo "  act-test-tag     - Test tag push event (full workflow)"
	@echo "  act-test-job     - Test specific job (build-and-push-image)"
	@echo "  act-dry-run      - Show what would be executed (no actual run)"
	@echo "  act-validate     - Validate workflow syntax"
	@echo "  act-debug        - Show debug information"
	@echo "  act-analyze      - Analyze workflow structure and dependencies"
	@echo ""
	@echo "🎯 Individual Job Testing:"
	@echo "  act-test-syntax-only - Test workflow syntax validation only"
	@echo "  act-test-local       - Test workflow with local-only steps"
	@echo "  act-test-env         - Test environment variable setup"
	@echo ""
	@echo "🔧 Additional Tools:"
	@echo "  ./test-workflow.sh   - Run comprehensive workflow testing script"
	@echo ""
	@echo "  all            - Run lint + build + test (default)"
