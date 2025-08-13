# Platform-specific base images for cross-compilation
# AMD64: Use Parity's unified CI image
# ARM64: Use Parity's cross-compilation image
FROM paritytech/ci-unified:latest AS base-amd64
FROM paritytech/xbuilder-aarch64-unknown-linux-gnu:latest AS base-arm64
# Choose base image based on target architecture
ARG TARGETARCH
FROM base-${TARGETARCH} AS base

WORKDIR /fennel

# Install cargo-chef and pre-fetch the wasm target once (only for AMD64, ARM64 image has it)
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "amd64" ]; then \
        echo "Setting up AMD64 build environment..." && \
        cargo install cargo-chef && \
        rustup target add wasm32-unknown-unknown \
    ; else \
        echo "Using ARM64 cross-compilation environment (tools pre-installed)..." \
    ; fi

# Optimize cargo for space and reduce compilation units
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV CARGO_INCREMENTAL=0
ENV CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
ENV CARGO_PROFILE_RELEASE_LTO=true
# Keep debug info for production debugging (default is 2)
# ENV CARGO_PROFILE_RELEASE_DEBUG=0  # Commented out - we need debug symbols
# Use balanced optimization level (3 is default, 's' optimizes for size while keeping performance)
ENV CARGO_PROFILE_RELEASE_OPT_LEVEL=s

# Planner stage - analyze dependencies (AMD64 only, ARM64 skips cargo-chef)
FROM base AS planner
ARG TARGETARCH
COPY . .
RUN if [ "$TARGETARCH" = "amd64" ]; then \
        cargo chef prepare --recipe-path recipe.json \
    ; else \
        # ARM64: cargo-chef not available, create empty recipe
        echo '{"dependencies": [], "features": []}' > recipe.json \
    ; fi

# Testing stage - skip tests in Docker build to save space
# Tests are already run in CI before Docker build
# FROM base AS tester
# COPY . .
# RUN cargo test --features=runtime-benchmarks

# --- Deterministic WASM runtime build using srtool (AMD64 only) -----------------
# Note: srtool doesn't support ARM64, so we create conditional stages
FROM docker.io/paritytech/srtool:1.84.1 AS srtool-amd64

# The srtool image expects the sources to live in /build
WORKDIR /build

# Copy the full workspace so that frame pallets & dependencies are available
COPY --chown=builder:builder . .

# Tell srtool which crate contains the runtime. Adjust these paths/names if you
# ever rename the runtime crate or move it to another folder.
ENV RUNTIME_DIR=runtime/fennel
ENV PACKAGE=fennel-node-runtime

# Build the runtime in deterministic mode. The build script lives inside the
# image under /scripts/build
RUN /srtool/build

# Create empty srtool stage for ARM64 (no srtool support)
FROM scratch AS srtool-arm64

# Select appropriate srtool stage based on target architecture
ARG TARGETARCH
FROM srtool-$TARGETARCH AS srtool

# The compact deterministic wasm will be available below (for AMD64 only).
ENV DETERMINISTIC_WASM_PATH=target/srtool/release/wbuild/fennel-node-runtime/fennel_node_runtime.compact.wasm

# Builder stage - build with cached dependencies
FROM base AS builder
ARG TARGETARCH
COPY --from=planner /fennel/recipe.json recipe.json

# Build dependencies first (this layer will be cached)
# For ARM64, cargo-chef might not be available, so we skip this optimization
RUN if [ "$TARGETARCH" = "amd64" ]; then \
        cargo chef cook --release --recipe-path recipe.json && \
        rm -rf /fennel/target/release/deps/*.rlib && \
        rm -rf /fennel/target/release/build && \
        find /fennel/target/release -type f -name "*.d" -delete \
    ; fi

# Now copy the actual source code and build
COPY . .

# Build with platform-specific configuration
RUN if [ "$TARGETARCH" = "arm64" ]; then \
        # ARM64 cross-compilation build with proper linker
        rustup target add aarch64-unknown-linux-gnu && \
        CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
        cargo build --locked --release --target aarch64-unknown-linux-gnu && \
        mkdir -p /fennel/target/release && \
        cp /fennel/target/aarch64-unknown-linux-gnu/release/fennel-node /fennel/target/release/fennel-node \
    ; else \
        # AMD64 native build with cleanup
        cargo build --locked --release && \
        rm -rf /fennel/target/release/deps/*.rlib && \
        rm -rf /fennel/target/release/build && \
        rm -rf /fennel/target/release/.fingerprint && \
        rm -rf /fennel/target/release/incremental && \
        find /fennel/target/release -type f -name "*.d" -delete \
    ; fi

# Runtime stage - final image with minimal components
# Use Ubuntu as base since parity/base-bin doesn't support ARM64
FROM ubuntu:22.04

# Copy the node binary
COPY --from=builder /fennel/target/release/fennel-node /usr/local/bin/fennel-node

ARG WASM_HASH=unknown
LABEL io.parity.srtool.wasm-hash=${WASM_HASH}

# OCI-compliant labels for container metadata and multi-arch manifest
LABEL org.opencontainers.image.title="Fennel Node" \
      org.opencontainers.image.description="Fennel Node w arm64 and amd64 capabilities." \
      org.opencontainers.image.vendor="Fennel Network" \
      org.opencontainers.image.authors="CorruptedAesthetic" \
      org.opencontainers.image.url="https://github.com/CorruptedAesthetic/fennel-solonet" \
      org.opencontainers.image.source="https://github.com/CorruptedAesthetic/fennel-solonet" \
      org.opencontainers.image.documentation="https://github.com/CorruptedAesthetic/fennel-solonet/blob/main/README.md" \
      org.opencontainers.image.licenses="Apache-2.0" \
      org.opencontainers.image.ref.name="fennel-solonet" \
      org.opencontainers.image.base.name="ubuntu:22.04"

# Install minimal runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /fennel fennel && \
	mkdir -p /data /fennel/.local/share && \
	chown -R fennel:fennel /data && \
	ln -s /data /fennel/.local/share/fennel && \
# check if executable works in this container
	/usr/local/bin/fennel-node --version

USER fennel

EXPOSE 9933 9944 30333 9615
# Removed ports:
# - 9930: Removed unless specific reverse-proxy need (ecosystem standard)
# - 30334: Removed - only needed for relay-within-relay processes
VOLUME ["/data"]

# Use node binary as entrypoint (Parity standard practice)
ENTRYPOINT ["/usr/local/bin/fennel-node"]
