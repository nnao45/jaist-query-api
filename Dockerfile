# syntax = tonistiigi/dockerfile:runmount20181002

# Setup Building Container
## Install Dependency Module
FROM rust:1.57.0-slim as builder

VOLUME ["/output", "/usr/local/cargo"]

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu \
    OPENSSL_INCLUDE_DIR=/usr/include/openssl \
    RUST_VERSION=%%RUST-VERSION%% \
    DEBIAN_FRONTEND=noninteractive
RUN set -eux; \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        libssl-dev \
        git \
        default-libmysqlclient-dev
RUN rustup default nightly-2021-12-16

## Build Cache Dependency Library
RUN mkdir /app
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/root/.cargo \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release -Z unstable-options --out-dir /output

FROM debian:11.1-slim as worker
ENV DEBIAN_FRONTEND=noninteractive
RUN set -eux; \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        libssl-dev \
        ca-certificates \
        tzdata \
        default-libmysqlclient-dev

RUN cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime && \
    echo "Asia/Tokyo" > /etc/timezone

## Copy App
COPY --from=builder /output/main .

## Run
CMD ["./main"]