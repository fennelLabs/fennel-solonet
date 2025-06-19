# Fennel Solonet Release Process

## Version Guidelines (Semantic Versioning)

### When to Bump Versions:

#### PATCH (0.1.0 → 0.1.1)
- Bug fixes
- Configuration updates (bootnodes, chain specs)
- Docker/CI improvements
- Documentation updates

#### MINOR (0.1.1 → 0.2.0)
- New pallets added
- New RPC methods
- Runtime upgrades (non-breaking)
- New features

#### MAJOR (0.2.0 → 1.0.0)
- Breaking runtime changes
- Storage migrations required
- Incompatible API changes
- Mainnet launch

## Release Process

### 1. Update Version
```bash
# Edit Charts/fennel-node/Chart.yaml
version: 0.1.1  # Bump this
appVersion: "0.1.1"  # And this
```

### 2. Commit and Push
```bash
git add Charts/fennel-node/Chart.yaml
git commit -m "chore: bump version to 0.1.1"
git push origin main
```

### 3. Create Release Tag
```bash
git tag -a v0.1.1 -m "Release v0.1.1: <description>"
git push origin v0.1.1
```

### 4. CI Automatically Creates:
- GitHub Release with Helm chart
- Docker image on ghcr.io
- Updates gh-pages Helm repository

## Release Checklist

- [ ] Runtime tests pass
- [ ] Benchmarks updated (if needed)
- [ ] Chain spec updated (if needed)
- [ ] Version bumped in Chart.yaml
- [ ] Release notes written
- [ ] Tag created and pushed

## Example Release Notes

```markdown
## What's Changed
- feat: Add XYZ pallet
- fix: Resolve issue with ABC
- chore: Update dependencies

## Runtime Hash
- Development: 0x123...
- Staging: 0x456...

## Docker Image
`ghcr.io/corruptedaesthetic/fennel-solonet:v0.1.1`

## Helm Chart
`helm upgrade fennel fennel/fennel-node --version 0.1.1`
``` 



graph TD
    A[1. Edit Code] --> B[2. Bump Chart.yaml version]
    B --> C[3. git push origin main]
    C --> D[CI: Build Docker image :latest]
    C --> E[CI: Package Helm chart]
    C --> F[CI: Skip GitHub Release<br/>version already exists]
    
    B --> G[4. git tag -a v0.1.1]
    G --> H[5. git push origin v0.1.1]
    H --> I[CI: Build Docker image :v0.1.1]
    H --> J[CI: Create GitHub Release v0.1.1]
    H --> K[CI: Attach Helm chart to Release]
    H --> L[CI: Update gh-pages index]
    
    style F fill:#ff9999
    style J fill:#99ff99
    style K fill:#99ff99


    --- further info ---
    Exactly! You've got it! 🎯 

## Yes, this is the FINAL step after ALL your changes

The process is:

### During Development (multiple commits):
```bash
# Make changes, commit, push as many times as needed
git add .
git commit -m "feat: add new function"
git push origin main

# More changes...
git add .
git commit -m "fix: resolve issue"
git push origin main

# Even more changes...
git add .
git commit -m "docs: update README"
git push origin main
```

### When Ready to Release (FINAL step):
```bash
# 1. Decide "OK, this is ready for a release"
# 2. Update version
vim Charts/fennel-node/Chart.yaml  # 0.1.0 → 0.1.1

# 3. Commit the version bump
git add Charts/fennel-node/Chart.yaml
git commit -m "chore: bump version to 0.1.1"
git push origin main

# 4. Create the release tag (the "v" indicates official release)
git tag -a v0.1.1 -m "Release v0.1.1: Enhanced trust pallet, fixed issues, updated docs"
git push origin v0.1.1
```

## The "v" prefix is a convention that:
- ✅ **Triggers** the release workflow
- ✅ **Creates** GitHub Release
- ✅ **Tags** Docker image with version
- ✅ **Publishes** Helm chart
- ✅ **Signals** to users "this is an official release"

## Think of it like:
- **Regular commits** = Work in progress
- **Tag with "v"** = "Ship it! 🚀"

So yes, you can make as many commits as you want during development. Only when you're ready to create an official release do you:
1. Bump the version
2. Create a tag with "v" prefix
3. Push the tag

This creates a clear distinction between development work and official releases!