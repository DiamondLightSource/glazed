FROM rust:1.91-slim AS build
WORKDIR /build

RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && \
    apt-get install -y musl-tools musl-dev && \
    update-ca-certificates

# Build an empty project with only the Cargo files to improve the cache
# performance of the container build. The src directory is expected to change
# most frequently invalidating later caches.
# This downloads and builds the dependencies early allowing built dependencies
# to be cached.
RUN mkdir src && echo 'fn main() {}' > src/main.rs
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release --target x86_64-unknown-linux-musl

COPY ./src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

ENTRYPOINT ["/build/target/x86_64-unknown-linux-musl/release/glazed"]
CMD ["serve"]
