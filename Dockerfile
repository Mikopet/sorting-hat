# Build stage
FROM rust:alpine AS build

ARG APP_NAME="sorting-hat"
ENV TARGET="x86_64-unknown-linux-musl"
ENV RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /app

RUN apk add --no-cache openssl-dev alpine-sdk clang musl-dev lld
RUN rustup target add x86_64-unknown-linux-musl

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/$APP_NAME bot

# Run stage
FROM alpine:latest

RUN apk add --no-cache libgcc

WORKDIR /app
COPY --from=build --chmod=755 /app/bot .
# COPY .env .

CMD ["/app/bot"]
