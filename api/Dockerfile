FROM rust:1.91.1-alpine3.20 AS build
RUN apk add build-base openssl-dev openssl-libs-static libssl3 libcrypto3

WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo fetch

COPY . .

RUN cargo build --bin standalone --release

FROM alpine:3.20

COPY --from=build /app/target/release/standalone ./cinescore-api

ENV ENVIRONMENT="production"

EXPOSE 8000
ENTRYPOINT ["/bin/sh", "-c", "./cinescore-api --bind 0.0.0.0 --port 8000"]

