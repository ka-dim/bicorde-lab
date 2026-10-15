# syntax=docker/dockerfile:1

FROM rust:1.85-bookworm AS builder

WORKDIR /build

COPY server/Cargo.toml server/Cargo.lock ./
COPY server/src ./src

RUN cargo build --release --all-features --locked

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /build/target/release/server /app/server

ENV BICORDE_ADDRESS=0.0.0.0:3000

EXPOSE 3000

USER 10001:10001

ENTRYPOINT ["/app/server"]
