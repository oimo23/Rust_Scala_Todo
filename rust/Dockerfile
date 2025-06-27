FROM rust:1.82 as builder

WORKDIR /app
COPY Cargo.toml .
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/todo-rust .

EXPOSE 8000

CMD ["./todo-rust"]