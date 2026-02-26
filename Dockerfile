FROM rust:1.84-bookworm AS builder

WORKDIR /build

# Copy workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY conduit-api/Cargo.toml conduit-api/Cargo.toml
COPY conduit-consumer/Cargo.toml conduit-consumer/Cargo.toml

# Create dummy sources to cache dependency builds
RUN mkdir -p conduit-api/src conduit-consumer/src \
    && echo "fn main() {}" > conduit-api/src/main.rs \
    && echo "fn main() {}" > conduit-consumer/src/main.rs \
    && cargo build --release -p conduit-api 2>/dev/null || true

# Copy real source and build
COPY conduit-api/src conduit-api/src
COPY conduit-api/config conduit-api/config
COPY conduit-consumer/src conduit-consumer/src
RUN cargo build --release -p conduit-api

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

RUN groupadd -g 1001 conduit && useradd -u 1001 -g conduit -m conduit

COPY --from=builder /build/target/release/conduit-api /usr/local/bin/conduit-api
COPY --from=builder /build/conduit-api/config /opt/conduit/config

RUN mkdir -p /mnt/merkql && chown conduit:conduit /mnt/merkql

USER conduit

ENV MERKQL_DATA_PATH=/mnt/merkql
ENV FUNCTIONS_CUSTOMHANDLER_PORT=3000

EXPOSE 3000

CMD ["conduit-api"]
