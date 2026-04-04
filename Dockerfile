FROM rust:1.94 AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
COPY patches ./patches

RUN cargo install patch-crate
RUN cargo patch-crate
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/grafbase-external-composition /usr/local/bin/grafbase-external-composition

EXPOSE 4000

CMD ["grafbase-external-composition"]
