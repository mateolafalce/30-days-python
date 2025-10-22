FROM rust:1.90.0 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/dayspython .
COPY markdown ./markdown
COPY static ./static
COPY images ./images

EXPOSE 5001
CMD ["./dayspython"]