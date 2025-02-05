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

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

RUN rm -rf /app/target/x86_64-unknown-linux-musl/release/build
RUN rm -rf /usr/local/cargo/registry

FROM --platform=linux/amd64 debian:bullseye-slim as litestream

WORKDIR /app

COPY litestream.yml litestream.yml

ENV LITESTREAM_VERSION=0.3.5

ADD https://github.com/benbjohnson/litestream/releases/download/v$LITESTREAM_VERSION/litestream-v$LITESTREAM_VERSION-linux-amd64.tar.gz /tmp/litestream.tar.gz
RUN tar -C /usr/local/bin -xzf /tmp/litestream.tar.gz

FROM --platform=linux/amd64 alpine:latest
WORKDIR /app
COPY --from=litestream /usr/local/bin/litestream /usr/local/bin/litestream
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api /app/app

COPY ./docker-entrypoint.sh /app/docker-entrypoint.sh

ENV GLIBC_VER=2.35-r0

RUN apk update --no-cache && \
  apk add --no-cache git curl binutils && \
  curl -sL https://alpine-pkgs.sgerrand.com/sgerrand.rsa.pub -o /etc/apk/keys/sgerrand.rsa.pub && \
  curl -sLO "https://github.com/sgerrand/alpine-pkg-glibc/releases/download/${GLIBC_VER}/glibc-${GLIBC_VER}.apk" && \
  curl -sLO "https://github.com/sgerrand/alpine-pkg-glibc/releases/download/${GLIBC_VER}/glibc-bin-${GLIBC_VER}.apk" && \
  apk add --force-overwrite --no-cache glibc-${GLIBC_VER}.apk glibc-bin-${GLIBC_VER}.apk && \
  rm -rf /var/cache/apk/*

EXPOSE 8080

CMD ["./app"]
