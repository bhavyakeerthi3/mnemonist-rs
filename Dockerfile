FROM rust:1-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY build.rs ./
COPY src ./src
RUN cargo build --release --no-default-features --bin mnemonist

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/mnemonist /usr/local/bin/mnemonist
ENTRYPOINT ["mnemonist"]
