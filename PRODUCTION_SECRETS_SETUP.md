# Production Secrets Setup Guide

This guide explains how to configure GitHub Secrets for production chainspec generation.

## 🔐 Required GitHub Secrets

The following secrets must be configured in your GitHub repository settings for production builds to work:

### Go to Repository Settings
1. Navigate to: `https://github.com/CorruptedAesthetic/fennel-solonet/settings/secrets/actions`
2. Click "New repository secret" for each of the following:

### Required Secrets

| Secret Name | Description | Example Format |
|-------------|-------------|----------------|
| `PROD_SUDO_SS58` | Production sudo account SS58 address | `5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV` |
| `PROD_VAL1_AURA_PUB` | Validator 1 AURA public key (hex) | `0x46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a` |
| `PROD_VAL1_GRANDPA_PUB` | Validator 1 GRANDPA public key (hex) | `0x345071da55e5dccefaaa440339415ef9f2663338a38f7da0df21be5ab4e055ef` |
| `PROD_VAL1_STASH_SS58` | Validator 1 stash account SS58 address | `5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV` |
| `PROD_VAL2_AURA_PUB` | Validator 2 AURA public key (hex) | `0x46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a` |
| `PROD_VAL2_GRANDPA_PUB` | Validator 2 GRANDPA public key (hex) | `0x345071da55e5dccefaaa440339415ef9f2663338a38f7da0df21be5ab4e055ef` |
| `PROD_VAL2_STASH_SS58` | Validator 2 stash account SS58 address | `5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV` |

## 🔑 Getting the Secret Values

### Option 1: From fennel-prod Repository
If you have the `fennel-prod` repository set up with Vault:

```bash
cd /path/to/fennel-prod
./environments/production/extract-github-secrets.sh
```

This script will output the exact values you need to copy into GitHub Secrets.

### Option 2: Manual Generation
If you need to generate new production keys:

```bash
# Generate validator 1 keys
fennel-node key generate --scheme sr25519 --output-type json > val1_aura.json
fennel-node key generate --scheme ed25519 --output-type json > val1_grandpa.json

# Generate validator 2 keys  
fennel-node key generate --scheme sr25519 --output-type json > val2_aura.json
fennel-node key generate --scheme ed25519 --output-type json > val2_grandpa.json

# Extract public keys and SS58 addresses from the JSON files
```

## 🚀 Testing the Setup

Once you've configured all the secrets:

1. **Create a release tag**:
   ```bash
   git tag -a fennel-node-0.5.0 -m "Production release v0.5.0"
   git push origin fennel-node-0.5.0
   ```

2. **Monitor the GitHub Actions workflow**:
   - Go to the Actions tab in your repository
   - Watch for the "Create and publish a Docker image" workflow
   - Verify it detects the production release and exports the secrets

3. **Check the artifacts**:
   - The workflow should generate production chainspecs
   - They should be included in the GitHub release
   - Verify the production chainspec contains your validator keys

## 🔒 Security Notes

- **Public Keys Only**: These secrets contain only public keys and SS58 addresses
- **Safe to Store**: Public keys are safe to store in GitHub Secrets
- **Private Keys**: Private validator keys remain secure in HashiCorp Vault
- **Runtime Injection**: Private keys are injected at runtime via Vault Agent

## 🔄 Migration Path

This GitHub Secrets approach provides:
1. **Immediate Solution**: Production chainspecs generated right away
2. **Security**: Public keys are safe in GitHub Secrets
3. **Simplicity**: No complex Vault OIDC authentication in CI/CD
4. **Vault Integration**: Private keys still managed via Vault for runtime

The compilation methodology remains the same - only the source of public keys changes from Vault to GitHub Secrets.

## ✅ Verification

After setup, your production builds should:
- ✅ Detect release tags correctly
- ✅ Export production environment variables from GitHub Secrets
- ✅ Build runtime with production keys embedded
- ✅ Generate production chainspecs
- ✅ Include chainspecs in GitHub releases
- ✅ Show "Production release detected" in workflow logs

---

**Next Steps**: Once this is working, you can optionally migrate back to Vault integration if needed, but GitHub Secrets provides a simpler and equally secure approach for public key material. 