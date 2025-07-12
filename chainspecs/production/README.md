# Fennel Production Chainspecs

This directory contains the production chainspecs for the Fennel blockchain network.

## 📋 Files

- `production-chainspec.json` - Human-readable production chain specification
- `production-raw.json` - SCALE-encoded raw production chainspec for node deployment

## 🏭 Production Configuration

### Chain Details
- **Chain ID**: `fennel_production`
- **Chain Type**: `Live` (Production)
- **Network Name**: Fennel Production Network

### Consensus
- **Block Production**: AURA (Authority Round)
- **Finality**: GRANDPA (GHOST-based Recursive Ancestor Deriving Prefix Agreement)

### Authority Configuration
The production chainspec contains **placeholder validator keys** that must be replaced during deployment:

⚠️ **Important Security Notes:**
- Placeholder keys are used for chainspec generation
- Real validator and bootnode keys are managed via HashiCorp Vault
- Production keys should be generated offline using air-gapped systems
- See `DOCUMENTATION/UPGRADETOPRODUCTION/ndoesessionkeygenerations/` for secure key generation procedures

## 🌐 Bootnode Architecture

Uses **external-only bootnode architecture** following Parity's best practices:
- Both internal and external validators connect to the same public bootnode endpoints
- Bootnode peer IDs are dynamically derived from GitHub secrets during CI
- DNS-based addressing: `bootnode1.fennel.network` and `bootnode2.fennel.network`

## 🔐 Production Deployment Requirements

### Prerequisites
1. **HashiCorp Vault Setup**: Secure key management system deployed
2. **Real Validator Keys**: Generated offline and stored in Vault
3. **Production Infrastructure**: Azure AKS cluster with proper security policies
4. **DNS Configuration**: Bootnode domains pointing to production load balancers

### Key Management
- **Validator Session Keys**: AURA (Sr25519) + GRANDPA (Ed25519) keys per validator
- **Bootnode Node Keys**: Ed25519 keys for libp2p networking
- **Storage**: All keys stored in Vault with KV v2 engine
- **Access Control**: Kubernetes service account-based authentication

## 🚀 Usage

### For Production Deployment
```bash
# The production chainspec is automatically referenced by fennel-prod Helm charts
# Example production values.yaml:
node:
  customChainspecUrl: "https://raw.githubusercontent.com/CorruptedAesthetic/fennel-solonet/main/chainspecs/production/production-raw.json"
  chain: "production"
```

### For External Validators
```bash
# Download and use the production chainspec
curl -O https://raw.githubusercontent.com/CorruptedAesthetic/fennel-solonet/main/chainspecs/production/production-raw.json

# Start validator with production chainspec
./fennel-node \
  --chain production-raw.json \
  --validator \
  --name "my-production-validator"
```

## 🔄 Generation Process

Production chainspecs are automatically generated during CI/CD:

1. **Trigger**: Only generated for release tags (`fennel-node-*`)
2. **Runtime**: Built deterministically with srtool
3. **Preset**: Uses `production` genesis preset from runtime
4. **Bootnodes**: Dynamically injected from GitHub secrets
5. **Validation**: SHA-256 computed for integrity verification

## 📦 Distribution

- **GitHub Releases**: Attached to release assets
- **GitHub Pages**: Available via raw URL
- **CI Artifacts**: Available in GitHub Actions artifacts

## 🔗 Related Documentation

- **Production Setup**: `/DOCUMENTATION/UPGRADETOPRODUCTION/`
- **Vault Integration**: `/DOCUMENTATION/UPGRADETOPRODUCTION/upgradenotes/creationofvaultkubernetes.md`
- **Key Generation**: `/DOCUMENTATION/UPGRADETOPRODUCTION/ndoesessionkeygenerations/`
- **Staging Environment**: `../staging/README.md`

---

**Generated**: Automatically during release builds  
**Last Updated**: See git commit history  
**Security Review**: Required before production deployment
