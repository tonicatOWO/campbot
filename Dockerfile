FROM rust:1.82-slim as builder

RUN apt-get update && apt-get install -y \
  pkg-config \
  libssl-dev \
  build-essential \
  cmake \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml ./
COPY Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
  ca-certificates \
  libssl3 \
  iputils-ping \
  curl \
  dnsutils \
  procps \
  && rm -rf /var/lib/apt/lists/* \
  && apt-get clean

RUN useradd -r -s /bin/false -u 1000 botuser

WORKDIR /app

COPY --from=builder /app/target/release/campbot /usr/local/bin/campbot
RUN chmod +x /usr/local/bin/campbot

RUN mkdir -p /app && chown botuser:botuser /app

USER botuser
CMD ["campbot"]

