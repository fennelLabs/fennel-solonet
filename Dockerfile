FROM docker.io/paritytech/ci-unified:latest as base

WORKDIR /fennel

# Install cargo-chef for faster builds
RUN cargo install cargo-chef

# Planner stage - analyze dependencies
FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Testing stage - run tests before building
FROM base AS tester
COPY . .
RUN cargo test --features=runtime-benchmarks

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

USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /fennel fennel && \
	mkdir -p /data /fennel/.local/share && \
	chown -R fennel:fennel /data && \
	ln -s /data /fennel/.local/share/fennel && \
# check if executable works in this container
	/usr/local/bin/fennel-node --version

# Copy chain specification file
COPY --from=builder /fennel/fennelSpecRaw.json /fennel/fennelSpecRaw.json

USER fennel

EXPOSE 9930 9933 9944 30333 30334
VOLUME ["/data"]

# Use node binary as entrypoint (Parity standard practice)
ENTRYPOINT ["/usr/local/bin/fennel-node"]
