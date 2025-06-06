#!/bin/bash

# Fennel Multi-Phase Deployment Script
# Implements immutable-base + overlay architecture

set -e

# Configuration
NAMESPACE=${NAMESPACE:-fennel}
RELEASE_NAME_VALIDATORS=${RELEASE_NAME_VALIDATORS:-fennel-solochain}
RELEASE_NAME_BOOTNODES=${RELEASE_NAME_BOOTNODES:-fennel-bootnodes}
HELM_REPO_NAME="parity"
HELM_REPO_URL="https://paritytech.github.io/helm-charts/"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

usage() {
    echo "Usage: $0 <phase> [command]"
    echo ""
    echo "Phases:"
    echo "  phase0                - Deploy dedicated bootnodes with static keys"
    echo "  phase1                - Deploy Alice bootstrap (single validator)"
    echo "  phase2                - Scale to Alice + Bob (2 validators)"
    echo "  generate-validator-keys - Generate static validator node keys"
    echo "  cleanup               - Clean up all deployments"
    echo ""
    echo "Testing modes:"
    echo "  test-quick            - Deploy testing overlay (fast iteration)"
    echo "  test-reset            - Reset test environment and redeploy"
    echo ""
    echo "Utility commands:"
    echo "  verify-secrets        - Check Kubernetes secrets exist"
    echo ""
    echo "Commands for phase0:"
    echo "  generate-keys - Generate static bootnode keys"
    echo "  deploy        - Deploy bootnodes"
    echo "  status        - Check bootnode status"
    echo ""
    exit 1
}

log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

error() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

check_prerequisites() {
    log "Checking prerequisites..."
    command -v kubectl >/dev/null 2>&1 || error "kubectl is required but not installed"
    command -v helm >/dev/null 2>&1 || error "helm is required but not installed"
    command -v jq >/dev/null 2>&1 || error "jq is required but not installed"
}

setup_helm_repo() {
    log "Setting up Helm repository..."
    helm repo add $HELM_REPO_NAME $HELM_REPO_URL >/dev/null 2>&1
    helm repo update >/dev/null 2>&1
    success "Helm repository configured"
}

create_namespace() {
    log "Creating namespace '$NAMESPACE'..."
    kubectl create namespace $NAMESPACE --dry-run=client -o yaml | kubectl apply -f - >/dev/null 2>&1
    success "Namespace ready"
}

generate_bootnode_keys() {
    log "Generating static bootnode keys..."
    
    # Check if subkey is available
    if command -v subkey >/dev/null 2>&1; then
        log "Using subkey to generate keys..."
        subkey generate-node-key --file boot0.key
        subkey generate-node-key --file boot1.key
    else
        log "Using openssl to generate keys (subkey not available)..."
        # Generate 32 random bytes and encode as hex
        openssl rand -hex 32 > boot0.key
        openssl rand -hex 32 > boot1.key
    fi
    
    success "Generated boot0.key and boot1.key"
    
    # Update the secret manifest
    log "Updating bootnode secret manifest..."
    BOOT0_B64=$(base64 -w 0 boot0.key)
    BOOT1_B64=$(base64 -w 0 boot1.key)
    
    sed -i "s/boot0.key: \"\"/boot0.key: $BOOT0_B64/" manifests/bootnode-static-keys-secret.yaml
    sed -i "s/boot1.key: \"\"/boot1.key: $BOOT1_B64/" manifests/bootnode-static-keys-secret.yaml
    
    success "Updated secret manifest"
    
    # Show secret verification info  
    log "Secret will contain the following files:"
    echo "📋 Bootnode key files:"
    echo "  - /keys/boot0.key (for pod-0)"
    echo "  - /keys/boot1.key (for pod-1)"
    echo "📝 Mount path: extraSecretMounts.secretName = bootnode-static-keys"
    
    # Clean up key files
    rm boot0.key boot1.key
    success "Cleaned up temporary key files"
}

generate_validator_keys() {
    log "Generating static validator node keys..."
    
    # Check if subkey is available
    if command -v subkey >/dev/null 2>&1; then
        log "Using subkey to generate validator keys..."
        subkey generate-node-key --file validator0.key  # Alice static key
        subkey generate-node-key --file validator1.key  # Bob static key
    else
        log "Using openssl to generate validator keys (subkey not available)..."
        openssl rand -hex 32 > validator0.key
        openssl rand -hex 32 > validator1.key
    fi
    
    success "Generated validator0.key and validator1.key"
    
    # Create validator node keys secret
    log "Creating validator node keys secret..."
    kubectl create secret generic validator-node-keys \
        --from-file=validator0.key \
        --from-file=validator1.key \
        --namespace $NAMESPACE \
        --dry-run=client -o yaml | kubectl apply -f -
    
    success "Created validator-node-keys secret"
    
    # Show secret verification
    log "Verifying secret contents..."
    echo "📋 Secret files available:"
    kubectl get secret validator-node-keys -n $NAMESPACE -o jsonpath='{.data}' | jq -r 'keys[]' | sed 's/^/  - \/keys\//'
    
    # Clean up key files
    rm validator0.key validator1.key
    success "Cleaned up temporary key files"
}

deploy_phase0() {
    log "=== PHASE 0: Deploying Dedicated Bootnodes ==="
    
    create_namespace
    setup_helm_repo
    
    # Apply security and resilience manifests
    log "Applying security and resilience manifests..."
    kubectl apply -f manifests/bootnode-static-keys-secret.yaml
    kubectl apply -f manifests/network-policy.yaml
    kubectl apply -f manifests/pod-disruption-budget.yaml
    success "Manifests applied"
    
    # Deploy bootnodes using overlay
    log "Deploying bootnodes with static keys..."
    helm upgrade --install $RELEASE_NAME_BOOTNODES $HELM_REPO_NAME/node \
        --namespace $NAMESPACE \
        --values values/values-base.yaml \
        --values values/bootnodes.yaml \
        --wait \
        --timeout 10m
    
    success "PHASE 0 COMPLETE: Dedicated bootnodes deployed!"
    
    # Show bootnode status
    kubectl get pods -n $NAMESPACE -l app.kubernetes.io/name=fennel-bootnodes
}

deploy_phase1() {
    log "=== PHASE 1: Alice Bootstrap Deployment ==="
    
    create_namespace
    setup_helm_repo
    
    # Deploy Alice with bootstrap overlay
    log "Deploying Alice (single validator bootstrap)..."
    helm upgrade --install $RELEASE_NAME_VALIDATORS $HELM_REPO_NAME/node \
        --namespace $NAMESPACE \
        --values values/values-base.yaml \
        --values values/bootstrap.yaml \
        --wait \
        --timeout 10m
    
    success "PHASE 1 COMPLETE: Alice bootstrap deployed!"
    
    # Show next steps
    echo ""
    warning "NEXT STEPS:"
    echo "1. Set up port forwarding: kubectl port-forward -n $NAMESPACE svc/$RELEASE_NAME_VALIDATORS-node 9944:9944"
    echo "2. Generate Alice's keys: curl -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"author_rotateKeys\"}' http://localhost:9944"
    echo "3. Register keys via Polkadot.js Apps: https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944"
    echo "4. Then run: $0 phase2"
}

deploy_phase2() {
    log "=== PHASE 2: Scaling to Alice + Bob ==="
    
    # Upgrade to 2 validators using scale overlay
    log "Scaling to 2 validators (Alice + Bob)..."
    helm upgrade $RELEASE_NAME_VALIDATORS $HELM_REPO_NAME/node \
        --namespace $NAMESPACE \
        --values values/values-base.yaml \
        --values values/scale-2.yaml \
        --wait \
        --timeout 10m
    
    success "PHASE 2 COMPLETE: Alice + Bob deployed!"
    
    # Show next steps
    echo ""
    warning "NEXT STEPS:"
    echo "1. Temporarily enable unsafe RPC for Bob: helm upgrade $RELEASE_NAME_VALIDATORS $HELM_REPO_NAME/node --reuse-values --set node.allowUnsafeRpcMethods=true -n $NAMESPACE"
    echo "2. Set up port forwarding to Bob: kubectl port-forward -n $NAMESPACE $RELEASE_NAME_VALIDATORS-node-1 9945:9944"
    echo "3. Generate Bob's keys: curl -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"author_rotateKeys\"}' http://localhost:9945"
    echo "4. Register Bob's keys via Polkadot.js Apps"
    echo "5. SECURITY: Disable unsafe RPC: helm upgrade $RELEASE_NAME_VALIDATORS $HELM_REPO_NAME/node --reuse-values --set node.allowUnsafeRpcMethods=false -n $NAMESPACE"
    echo "6. Ready for Phase 3 (external validators)"
}

cleanup() {
    log "=== CLEANUP: Removing all deployments ==="
    
    helm uninstall $RELEASE_NAME_VALIDATORS -n $NAMESPACE 2>/dev/null || true
    helm uninstall $RELEASE_NAME_BOOTNODES -n $NAMESPACE 2>/dev/null || true
    kubectl delete namespace $NAMESPACE 2>/dev/null || true
    
    success "Cleanup complete!"
}

status_phase0() {
    log "=== BOOTNODE STATUS ==="
    echo ""
    kubectl get pods -n $NAMESPACE -l app.kubernetes.io/name=fennel-bootnodes
    echo ""
    kubectl get svc -n $NAMESPACE -l app.kubernetes.io/name=fennel-bootnodes
}

deploy_test_quick() {
    log "=== TESTING MODE: Quick Multi-Validator Deployment ==="
    
    create_namespace
    setup_helm_repo
    
    # Generate keys if they don't exist
    if ! kubectl get secret validator-node-keys -n $NAMESPACE >/dev/null 2>&1; then
        log "Generating validator keys for testing..."
        generate_validator_keys
    fi
    
    # Deploy with testing overlay (maintains production patterns but faster iteration)
    log "Deploying testing environment (Alice + Bob with static keys)..."
    helm upgrade --install $RELEASE_NAME_VALIDATORS $HELM_REPO_NAME/node \
        --namespace $NAMESPACE \
        --values values/values-base.yaml \
        --values values/testing.yaml \
        --wait \
        --timeout 10m
    
    success "TESTING MODE COMPLETE: Alice + Bob deployed for testing!"
    
    # Show testing access info
    echo ""
    warning "TESTING ACCESS:"
    echo "1. Port forward: kubectl port-forward -n $NAMESPACE svc/$RELEASE_NAME_VALIDATORS-node 9944:9944"
    echo "2. Alice: ws://localhost:9944"
    echo "3. Bob: kubectl port-forward -n $NAMESPACE $RELEASE_NAME_VALIDATORS-node-1 9945:9944"
    echo "4. Unsafe RPC enabled for automated testing"
    echo "5. Static keys: deterministic peer IDs for reproducible tests"
}

deploy_test_reset() {
    log "=== TESTING MODE: Clean Reset ==="
    
    # Clean up existing deployment
    helm uninstall $RELEASE_NAME_VALIDATORS -n $NAMESPACE 2>/dev/null || true
    
    # Wait for pods to terminate
    log "Waiting for pods to terminate..."
    sleep 10
    
    # Redeploy testing environment
    deploy_test_quick
    
    success "TEST RESET COMPLETE: Fresh testing environment ready!"
}

verify_secrets() {
    log "=== VERIFYING KUBERNETES SECRETS ==="
    
    log "Checking for required secrets in namespace '$NAMESPACE'..."
    
    # Check bootnode keys
    if kubectl get secret bootnode-static-keys -n $NAMESPACE >/dev/null 2>&1; then
        success "✅ bootnode-static-keys secret exists"
        echo "📋 Contains files:"
        kubectl get secret bootnode-static-keys -n $NAMESPACE -o jsonpath='{.data}' | jq -r 'keys[]' | sed 's/^/  - \/keys\//'
    else
        warning "❌ bootnode-static-keys secret missing"
        echo "   Run: ./deploy-phases.sh phase0 generate-keys"
    fi
    
    # Check validator keys
    if kubectl get secret validator-node-keys -n $NAMESPACE >/dev/null 2>&1; then
        success "✅ validator-node-keys secret exists"
        echo "📋 Contains files:"
        kubectl get secret validator-node-keys -n $NAMESPACE -o jsonpath='{.data}' | jq -r 'keys[]' | sed 's/^/  - \/keys\//'
    else
        warning "❌ validator-node-keys secret missing (optional)"
        echo "   Run: ./deploy-phases.sh generate-validator-keys"
    fi
    
    echo ""
    log "Secret verification complete!"
}

# Main logic
case "${1:-}" in
    "phase0")
        case "${2:-deploy}" in
            "generate-keys")
                generate_bootnode_keys
                ;;
            "deploy")
                deploy_phase0
                ;;
            "status")
                status_phase0
                ;;
            *)
                error "Invalid phase0 command. Use: generate-keys, deploy, or status"
                ;;
        esac
        ;;
    "generate-validator-keys")
        check_prerequisites
        create_namespace
        generate_validator_keys
        ;;
    "phase1")
        check_prerequisites
        deploy_phase1
        ;;
    "phase2")
        check_prerequisites
        deploy_phase2
        ;;
    "cleanup")
        cleanup
        ;;
    "test-quick")
        check_prerequisites
        deploy_test_quick
        ;;
    "test-reset")
        check_prerequisites
        deploy_test_reset
        ;;
    "verify-secrets")
        check_prerequisites
        create_namespace
        verify_secrets
        ;;
    *)
        usage
        ;;
esac 