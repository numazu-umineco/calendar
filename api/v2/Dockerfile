FROM --platform=linux/amd64 rust:1.82 as builder
WORKDIR /app

RUN apt-get update && apt-get install -y \
  musl-tools musl-dev

RUN rustup target add x86_64-unknown-linux-musl

ENV PKG_CONFIG_ALLOW_CROSS=1
ENV OPENSSL_STATIC=1
ENV CC_x86_64_unknown_linux_musl=musl-gcc
ENV AR_x86_64_unknown_linux_musl=ar
ENV RANLIB_x86_64_unknown_linux_musl=ranlib
ENV RUSTFLAGS="-C target-feature=+crt-static -C link-args=-lm"

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

RUN rm -rf /app/target/x86_64-unknown-linux-musl/release/build
RUN rm -rf /usr/local/cargo/registry

RUN mkdir -p storage && touch storage/.keep
RUN mkdir -p gcp && touch gcp/.keep

FROM alpine:latest AS certs
RUN apk add --no-cache ca-certificates

FROM --platform=linux/amd64 scratch
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api /app/app
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

COPY --from=builder /app/storage /storage
COPY --from=builder /app/gcp /gcp
EXPOSE 8080
CMD ["./app", "run"]
