FROM rust:latest AS builder

WORKDIR /usr/src/service1

RUN apt-get update && apt-get install -y \
    pkg-config && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/service1/target/release/service1 /app/service1

EXPOSE 8199

CMD ["./service1"]