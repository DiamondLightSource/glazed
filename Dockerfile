FROM rust:1.87.0-slim as build

WORKDIR /build

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./config.toml ./config.toml

RUN cargo build --release --target x86_64-unknown-linux-musl

CMD ["glazed serve"]