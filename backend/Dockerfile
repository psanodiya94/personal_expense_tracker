FROM rust:1.75 AS builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml ./
COPY backend ./backend

# Build the application
WORKDIR /app/backend
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/backend/target/release/expense-tracker-backend /app/expense-tracker-backend
COPY --from=builder /app/backend/migrations /app/migrations

ENV RUST_LOG=info

EXPOSE 3000

CMD ["/app/expense-tracker-backend"]
