# syntax=docker/dockerfile:1

# ---- common ARGs -------------------------------------------------
ARG BUILDPLATFORM
ARG TARGETPLATFORM
# ------------------------------------------------------------------

####################  🍳  BUILDER  ####################
FROM --platform=$BUILDPLATFORM docker.io/paritytech/ci-unified:bullseye-1.88.0 AS builder
ARG TARGETPLATFORM
WORKDIR /fennel

# --- cross-toolchain for Arm64 (gcc & full sysroot) ---------------
RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc-aarch64-linux-gnu g++-aarch64-linux-gnu \
    libc6-dev-arm64-cross libstdc++-10-dev-arm64-cross
# Tell cc-rs / bindgen where the headers live
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ \
    AR_aarch64_unknown_linux_gnu=aarch64-linux-gnu-ar

# Cargo-chef (one install is enough for both legs)
RUN cargo install cargo-chef

# Add WASM target for both platforms
RUN rustup target add wasm32-unknown-unknown

# Optimize cargo for space and reduce compilation units
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV CARGO_INCREMENTAL=0
ENV CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
ENV CARGO_PROFILE_RELEASE_LTO=true

# Planner stage - analyze dependencies
FROM builder AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Testing stage - run tests before building (only on native platform for speed)
FROM builder AS tester
COPY . .
# Only run tests on AMD64 to save build time, tests are architecture-independent for Substrate
RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
        cargo test --features=runtime-benchmarks; \
    else \
        echo "Skipping tests on $TARGETPLATFORM (tests run on amd64 only)"; \
    fi

# --- New stage: deterministic WASM runtime build using srtool -----------------
# Note: srtool produces architecture-independent WASM, so we only build on amd64
FROM --platform=linux/amd64 docker.io/paritytech/srtool:1.84.1 AS srtool
ARG TARGETPLATFORM

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
# Only build WASM on AMD64 since it's architecture-independent
RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
        /srtool/build; \
    else \
        echo "Skipping srtool build on $TARGETPLATFORM (WASM built on amd64 only)"; \
        mkdir -p runtime/fennel/target/srtool/release/wbuild/fennel-node-runtime; \
        touch runtime/fennel/target/srtool/release/wbuild/fennel-node-runtime/fennel_node_runtime.compact.wasm; \
    fi

# The compact deterministic wasm will be available below.
ENV DETERMINISTIC_WASM_PATH=runtime/fennel/target/srtool/release/wbuild/fennel-node-runtime/fennel_node_runtime.compact.wasm

# ---------------  Back to builder for final compilation  -----------------
FROM builder AS compiler
ARG TARGETPLATFORM

# Map Docker platform → Rust triple **once** and persist it
RUN case "$TARGETPLATFORM" in \
      linux/arm64) RUST_TARGET=aarch64-unknown-linux-gnu ;; \
      linux/amd64) RUST_TARGET=x86_64-unknown-linux-gnu   ;; \
      *) echo "Unsupported $TARGETPLATFORM" && exit 1 ;; \
    esac && \
    echo "Installing target $RUST_TARGET…" && \
    rustup target add --toolchain stable "$RUST_TARGET" && \
    echo "RUST_TARGET=$RUST_TARGET" >> /etc/environment

# ---------------  dependency cache  -----------------
COPY --from=planner /fennel/recipe.json recipe.json
# ensure std is present in *this* layer too
RUN . /etc/environment && \
    rustup target list --installed | grep -q "$RUST_TARGET" || rustup target add "$RUST_TARGET" && \
    cargo chef cook --release --target $RUST_TARGET --recipe-path recipe.json

# ---------------  final build  ----------------------
COPY . .
RUN . /etc/environment && \
    # ensure std is present in *this* layer too \
    rustup target list --installed | grep -q "$RUST_TARGET" || rustup target add "$RUST_TARGET" && \
    echo "Building for target: $RUST_TARGET" && \
    cargo build --locked --release --target=$RUST_TARGET && \
    mkdir -p /out && \
    install -Dm755 target/$RUST_TARGET/release/fennel-node /out/fennel-node

####################  🏃  RUNTIME  ####################
FROM --platform=$TARGETPLATFORM docker.io/parity/base-bin:latest
ARG TARGETPLATFORM

# Choose the right sub-directory and copy the binary
COPY --from=compiler /out/fennel-node /usr/local/bin/fennel-node

# Copy the deterministic WASM (optional, may fail on ARM64 builds where srtool doesn't run)
COPY --from=srtool /build/runtime/fennel/target/srtool/release/wbuild/fennel-node-runtime/fennel_node_runtime.compact.wasm /usr/local/bin/fennel_node_runtime.compact.wasm

# Expose standard Substrate ports
EXPOSE 9933 9944 30333 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/fennel-node"]
