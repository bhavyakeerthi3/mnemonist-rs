FROM rust:1-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY build.rs ./
COPY src ./src
COPY web ./web
RUN cargo build --release --no-default-features --bin mnemonist

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/mnemonist /usr/local/bin/mnemonist
EXPOSE 8787
ENTRYPOINT ["mnemonist"]
