FROM docker.io/paritytech/ci-unified:latest as base

WORKDIR /fennel

# Install cargo-chef and pre-fetch the wasm target once (minimal profile saves disk space)
RUN cargo install cargo-chef \
    && rustup target add wasm32-unknown-unknown --toolchain 1.84.1-x86_64-unknown-linux-gnu

# Avoid extra git clone storage in cargo registry (optional but helps disk)
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# Planner stage - analyze dependencies
FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Testing stage - run tests before building
FROM base AS tester
COPY . .
RUN cargo test --features=runtime-benchmarks

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
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --locked --release


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
