#!/bin/bash

# Script to create Kubernetes secret from chainspec file
# This secret will be mounted into the pods

NAMESPACE=${NAMESPACE:-default}
SECRET_NAME=${SECRET_NAME:-fennel-chainspec}
CHAINSPEC_FILE=${CHAINSPEC_FILE:-../fennelSpecRaw.json}

# Check if chainspec file exists
if [ ! -f "$CHAINSPEC_FILE" ]; then
    echo "Error: Chainspec file not found at $CHAINSPEC_FILE"
    echo "Please ensure fennelSpecRaw.json exists in the parent directory or specify CHAINSPEC_FILE env var"
    exit 1
fi

echo "Creating Kubernetes secret '$SECRET_NAME' in namespace '$NAMESPACE' from chainspec file..."

# For large files, create the secret using --from-file without dry-run
kubectl create secret generic $SECRET_NAME \
    --from-file=fennelSpec.json=$CHAINSPEC_FILE \
    --namespace=$NAMESPACE \
    2>/dev/null || \
kubectl create secret generic $SECRET_NAME \
    --from-file=fennelSpec.json=$CHAINSPEC_FILE \
    --namespace=$NAMESPACE \
    --dry-run=client -o yaml | \
    kubectl replace -f -

echo "Secret created/updated successfully!"
echo ""
echo "To verify: kubectl get secret $SECRET_NAME -n $NAMESPACE"
echo "To check size: kubectl get secret $SECRET_NAME -n $NAMESPACE -o json | jq '.data | length'" 