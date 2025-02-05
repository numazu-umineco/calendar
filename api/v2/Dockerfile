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
ENV RUSTFLAGS="-C target-feature=-crt-static -C link-args=-lm"

COPY Cargo.toml Cargo.lock ./
COPY ./src/ /app/src/

RUN cargo build --release --target x86_64-unknown-linux-musl

RUN rm -rf /app/target/x86_64-unknown-linux-musl/release/build
RUN rm -rf /usr/local/cargo/registry

FROM --platform=linux/amd64 debian:bullseye-slim

WORKDIR /app

COPY litestream.yml /app/db/litestream.yml

ENV LITESTREAM_VERSION=0.3.5

ADD https://github.com/benbjohnson/litestream/releases/download/v$LITESTREAM_VERSION/litestream-v$LITESTREAM_VERSION-linux-amd64.tar.gz /tmp/litestream.tar.gz
RUN tar -C /usr/local/bin -xzf /tmp/litestream.tar.gz \
  && rm /tmp/litestream.tar.gz

# 正しいバイナリファイル名を指定します
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api /app/app

COPY ./docker-entrypoint.sh /app/docker-entrypoint.sh

EXPOSE 8080

CMD ["app"]
