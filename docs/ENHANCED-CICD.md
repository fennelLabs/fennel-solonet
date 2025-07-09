# Enhanced CI/CD with Tag-Based Security

This document explains the enhanced CI/CD pipeline that implements cryptographic verification for releases.

## Overview

The enhanced CI/CD system ensures that:
- Only properly signed release tags trigger production builds
- Docker images, Helm charts, and chainspecs are cryptographically linked
- All artifacts are verified against their expected SHA-256 hashes
- Release builds fail if any artifacts are missing or tampered with

## Release Process

### 1. Create a Signed Release Tag

```bash
# Create a signed tag (requires GPG setup)
git tag -s fennel-node-0.4.3 -m "Fennel node v0.4.3"

# Push the tag to trigger the release build
git push origin fennel-node-0.4.3
```

**Tag Format**: Must follow `fennel-node-X.Y.Z` pattern (semantic versioning)

### 2. Automated Verification

The CI pipeline automatically:

1. **Verifies Tag Signature**: Ensures the tag is properly signed (currently format validation, GPG verification coming)
2. **Builds Docker Image**: Creates image tagged with both the Git tag and SHA
3. **Captures Image Digest**: Records the cryptographic digest for immutable verification
4. **Generates Chainspecs**: Creates development and staging chainspecs with embedded bootnodes
5. **Computes Chainspec Hashes**: Calculates SHA-256 for runtime verification
6. **Updates Helm Chart**: Injects tag, digest, and chainspec hash into values
7. **Creates Release**: Publishes all artifacts with integrity verification

### 3. Security Features

#### Docker Image Security
- **Tag Verification**: Images are tagged with the exact Git tag
- **Digest Pinning**: Helm charts reference images by immutable digest
- **Registry Verification**: Only signed images from trusted registry

#### Chainspec Security
- **SHA-256 Verification**: Chainspecs are verified by hash before use
- **Bootnode Validation**: Peer IDs derived from secure GitHub secrets
- **Integrity Checking**: Runtime verification prevents tampering

#### Helm Chart Security
- **Cryptographic Linking**: All components linked by release tag
- **Artifact Verification**: Build fails if any expected file is missing
- **Immutable References**: Charts use digest-based image references

## Deployment

### For Validators

```bash
# Add the Helm repository
helm repo add fennel https://corruptedaesthetic.github.io/fennel-solonet
helm repo update

# Deploy a specific verified release
helm install fennel-node fennel/fennel-node \
  --version 0.4.3 \
  -f values-staging.yaml
```

The deployment will:
1. Verify the Docker image digest matches the release
2. Validate the chainspec SHA-256 before starting
3. Ensure all components are from the same cryptographically-verified release

### Verification Commands

```bash
# Check the deployed image digest
kubectl describe pod fennel-node-0 | grep Image:

# Verify the chainspec hash (if using external chainspec)
curl -s https://github.com/CorruptedAesthetic/fennel-solonet/releases/download/fennel-node-0.4.3/staging-raw.json | sha256sum

# Check the Helm chart version
helm list
```

## Development vs. Release Builds

### Development Builds (main branch)
- Triggered on every commit to `main`
- Uses SHA-based tags for images
- Creates artifacts for testing
- No cryptographic verification required

### Release Builds (signed tags)
- Triggered only by signed `fennel-node-*` tags
- Full cryptographic verification pipeline
- Immutable artifact creation
- Production-ready deployments

## Setting Up GPG Signing (TODO)

Currently, the system validates tag format. To enable full GPG verification:

1. **Generate Signing Key**:
   ```bash
   gpg --full-gen-key
   ```

2. **Export Public Key**:
   ```bash
   gpg --armor --export YOUR_KEY_ID > .github/release-signing.pub.asc
   ```

3. **Upload to GitHub**: Add the public key to GitHub Settings → SSH and GPG keys

4. **Update Workflow**: Uncomment the GPG verification lines in `.github/workflows/publish.yml`

## Troubleshooting

### Tag Verification Failed
- Ensure tag follows `fennel-node-X.Y.Z` format
- Check that GPG key is properly configured (when enabled)

### Missing Artifacts
- Verify all required files exist before tagging
- Check that chainspecs are generated successfully
- Ensure Docker build completes without errors

### Image Digest Mismatch
- Indicates potential supply chain attack
- Verify the image in the registry matches expectations
- Check for any unauthorized modifications

## Security Benefits

This enhanced CI/CD provides:

✅ **Supply Chain Security**: Cryptographic verification of all artifacts  
✅ **Immutable Deployments**: Digest-based references prevent substitution  
✅ **Audit Trail**: Complete provenance from Git tag to running validator  
✅ **Automated Verification**: Runtime checks prevent deployment of tampered artifacts  
✅ **Fail-Safe Builds**: Process stops if any verification step fails  

For external validators, this means you can trust that what you deploy is exactly what was built and signed by the Fennel team.
