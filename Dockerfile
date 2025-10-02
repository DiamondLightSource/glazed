FROM rust:1.90-slim as build
WORKDIR /build

RUN rustup target add x86_64-unknown-linux-musl

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./config.toml ./config.toml

RUN cargo build --release --target x86_64-unknown-linux-musl

CMD ["glazed serve"]