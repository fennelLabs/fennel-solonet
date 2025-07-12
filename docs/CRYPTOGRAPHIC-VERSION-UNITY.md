```mermaid
graph TB
    %% Input triggers and verification
    T[Signed Git Tag<br/>fennel-node-X.Y.Z] --> GV{GPG Signature<br/>Verification}
    GV -->|✅ Valid| CI[CI/CD Pipeline]
    GV -->|❌ Invalid| FAIL[Build Fails]
    
    %% Version extraction and propagation
    CI --> VE[Version Extraction<br/>X.Y.Z from tag]
    VE --> RT[Runtime Build<br/>srtool]
    VE --> CS[Chainspec Generation]
    VE --> DI[Docker Image Build]
    VE --> HC[Helm Chart Update]
    
    %% Runtime and chainspec processing
    RT --> WH[Wasm Hash<br/>sha256]
    CS --> CSH[Chainspec SHA-256<br/>dev_sha + staging_sha]
    
    %% Docker image processing
    DI --> IT[Image Tag<br/>fennel-node-X.Y.Z]
    DI --> ID[Image Digest<br/>sha256:abc123...]
    
    %% Helm chart version unity
    HC --> CV[Chart Version<br/>X.Y.Z]
    HC --> CAV[Chart AppVersion<br/>X.Y.Z]
    
    %% Cryptographic linking in Helm values
    IT --> HV[Helm Values Update]
    ID --> HV
    CSH --> HV
    VE --> HV
    
    HV --> BV[Base values.yaml<br/>image.tag: X.Y.Z<br/>image.digest: sha256:...]
    HV --> SV[Staging values.yaml<br/>image.tag: X.Y.Z<br/>image.digest: sha256:...<br/>customChainspecSha256: abc123<br/>releaseTag: fennel-node-X.Y.Z]
    
    %% Template rendering with digest-aware image helper
    BV --> TH[{{fennel-node.image}} Helper]
    SV --> TH
    TH --> IR[Image Reference<br/>repo@sha256:digest OR repo:tag]
    
    %% Release artifact creation
    IT --> RA[Release Artifacts]
    ID --> RA
    WH --> RA
    CSH --> RA
    CV --> RA
    
    RA --> GR[GitHub Release<br/>• fennel-node-X.Y.Z.tgz<br/>• development.json + raw<br/>• staging-chainspec.json + raw<br/>• image-info.txt]
    RA --> HR[Helm Repository<br/>Chart Releaser]
    
    %% Deployment verification
    GR --> DV[Deployment Verification]
    HR --> DV
    DV --> VER[Runtime Verification<br/>• Image digest match<br/>• Chainspec SHA-256 match<br/>• Release tag consistency]
    
    %% Security guarantees
    VER --> SG[Security Guarantees<br/>✅ Cryptographic version unity<br/>✅ Immutable artifact references<br/>✅ Tamper detection<br/>✅ Reproducible deployments]
    
    %% Styling
    classDef input fill:#e1f5fe,stroke:#01579b,stroke-width:2px
    classDef process fill:#f3e5f5,stroke:#4a148c,stroke-width:2px
    classDef hash fill:#fff3e0,stroke:#e65100,stroke-width:2px
    classDef artifact fill:#e8f5e8,stroke:#1b5e20,stroke-width:2px
    classDef security fill:#ffebee,stroke:#b71c1c,stroke-width:2px
    classDef fail fill:#ffcdd2,stroke:#d32f2f,stroke-width:3px
    
    class T,GV input
    class CI,VE,RT,CS,DI,HC,HV,TH,DV process
    class WH,CSH,IT,ID,CV,CAV hash
    class BV,SV,IR,RA,GR,HR artifact
    class VER,SG security
    class FAIL fail
```

## Cryptographic Version Unity Architecture

This diagram illustrates how the fennel-solonet CI/CD pipeline enforces cryptographic version unity across all release artifacts. The system ensures that every component in a deployment can be traced back to a single, verified Git tag through an unbroken chain of cryptographic hashes and version references.

### Key Security Features

1. **Single Source of Truth**: The signed Git tag `fennel-node-X.Y.Z` is the authoritative version source
2. **Cryptographic Verification**: GPG signature verification ensures tag authenticity
3. **Hash Propagation**: SHA-256 hashes link runtime, chainspecs, and Docker images
4. **Digest-Based References**: Helm charts use immutable Docker image digests
5. **Comprehensive Verification**: Deployment verifies all hashes match expected values

### Version Unity Enforcement

- **Chart Version** = **App Version** = **Release Tag Version** = `X.Y.Z`
- **Docker Image Tag** = `fennel-node-X.Y.Z`
- **Docker Image Digest** = immutable `sha256:...` reference
- **Chainspec SHA-256** = runtime verification hash
- **Release Tag** = Git tag embedded in Helm values

This architecture prevents version drift, ensures reproducible deployments, and provides cryptographic proof that all artifacts belong to the same verified release.
