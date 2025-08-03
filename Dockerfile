FROM docker.io/paritytech/ci-unified:latest as base

WORKDIR /fennel

# Install cargo-chef and pre-fetch the wasm target once
RUN cargo install cargo-chef \
    && rustup target add wasm32-unknown-unknown

# Optimize cargo for space and reduce compilation units
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV CARGO_INCREMENTAL=0
ENV CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
ENV CARGO_PROFILE_RELEASE_LTO=true
# Keep debug info for production debugging (default is 2)
# ENV CARGO_PROFILE_RELEASE_DEBUG=0  # Commented out - we need debug symbols
# Use balanced optimization level (3 is default, 's' optimizes for size while keeping performance)
ENV CARGO_PROFILE_RELEASE_OPT_LEVEL=s

# Planner stage - analyze dependencies
FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Testing stage - skip tests in Docker build to save space
# Tests are already run in CI before Docker build
# FROM base AS tester
# COPY . .
# RUN cargo test --features=runtime-benchmarks

# --- New stage: deterministic WASM runtime build using srtool -----------------
FROM docker.io/paritytech/srtool:1.84.1 AS srtool

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

# The compact deterministic wasm will be available below.
ENV DETERMINISTIC_WASM_PATH=target/srtool/release/wbuild/fennel-node-runtime/fennel_node_runtime.compact.wasm

# Builder stage - build with cached dependencies
FROM base AS builder
COPY --from=planner /fennel/recipe.json recipe.json

# Build dependencies first (this layer will be cached)
RUN cargo chef cook --release --recipe-path recipe.json && \
    # Clean up immediately after cooking dependencies
    rm -rf /fennel/target/release/deps/*.rlib && \
    rm -rf /fennel/target/release/build && \
    find /fennel/target/release -type f -name "*.d" -delete

# Now copy the actual source code and build
COPY . .

# Build with cleanup in the same RUN to save space
RUN cargo build --locked --release && \
    # Clean up build artifacts immediately to free space
    rm -rf /fennel/target/release/deps/*.rlib && \
    rm -rf /fennel/target/release/build && \
    rm -rf /fennel/target/release/.fingerprint && \
    rm -rf /fennel/target/release/incremental && \
    # Keep only the final binary
    find /fennel/target/release -type f -name "*.d" -delete
    # Note: We don't strip the binary to preserve debug symbols for production debugging

# Runtime stage - final image with minimal components
FROM docker.io/parity/base-bin:latest

# Copy the node binary
COPY --from=builder /fennel/target/release/fennel-node /usr/local/bin/fennel-node

# Copy the deterministic wasm compiled with srtool (optional but convenient for
# governance upgrades & CI verification)
COPY --from=srtool /build/runtime/fennel/target/srtool/release/wbuild/fennel-node-runtime/fennel_node_runtime.compact.wasm /usr/local/bin/fennel_node_runtime.compact.wasm
RUN test -f /usr/local/bin/fennel_node_runtime.compact.wasm

ARG WASM_HASH=unknown
LABEL io.parity.srtool.wasm-hash=${WASM_HASH}

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
