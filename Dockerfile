FROM rust:1.74 as base
WORKDIR /app
RUN DEBIAN_FRONTEND=noninteractive \
    apt-get update -y && \
    ln -fs /usr/share/zoneinfo/America/New_York /etc/localtime && \
    apt-get install -y tzdata && \
    dpkg-reconfigure --frontend noninteractive tzdata && \
    apt-get install unzip curl build-essential protobuf-compiler -y && \
    apt-get install clang libclang-dev libclang1 llvm llvm-dev clang-tools -y && \
    apt-get upgrade -y

RUN cargo install cargo-chef

FROM base as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base as builder
COPY --from=planner /app/recipe.json recipe.json
COPY --from=planner /app/fennelSpecRaw.json fennelSpecRaw.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM base as runtime
COPY --from=builder /app/target/release/fennel-node /app/fennel-node
COPY --from=builder /app/fennelSpecRaw.json fennelSpecRaw.json
COPY --from=builder /app/chain-init.sh chain-init.sh
COPY --from=builder /app/peer-1-init.sh peer-1-init.sh
COPY --from=builder /app/peer-2-init.sh peer-2-init.sh
RUN /app/fennel-node --version
EXPOSE 9930 9333 9944 30333 30334
