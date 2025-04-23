# Build stage
FROM rust:1.86-slim AS builder

WORKDIR /app

# Copy the Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Copy the actual source code
COPY src/ ./src/

# Build the application in release mode
RUN cargo build --release

# Runtime stage
# Ubuntu 22.04 has GLIBC 2.35, which is newer than the required 2.33
FROM ubuntu:25.04

WORKDIR /app

# Install SSL certificates and other runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/connect4 ./

# Expose the port the app runs on
EXPOSE 8080

# Run the application
CMD ["./connect4", "server"]