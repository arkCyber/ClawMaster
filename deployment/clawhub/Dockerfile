# ClawHub Server Dockerfile

# Build stage
FROM rust:1.75-bookworm as builder

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY apps ./apps

# Build release binary
RUN cargo build -p clawhub-server --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 clawhub

# Copy binary
COPY --from=builder /build/target/release/clawhub-server /app/clawhub-server

# Set ownership
RUN chown -R clawhub:clawhub /app

# Switch to app user
USER clawhub

# Create data directory
RUN mkdir -p /home/clawhub/data

WORKDIR /app

# Expose port
EXPOSE 8080

# Set environment
ENV DATABASE_PATH=/home/clawhub/data/clawhub.db
ENV BIND_ADDR=0.0.0.0:8080
ENV RUST_LOG=info

# Run server
CMD ["/app/clawhub-server"]
