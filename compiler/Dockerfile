# Stage 1: Build environment
FROM rust:1.71-slim-bookworm as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new user and group with fixed IDs
RUN groupadd -g 10001 kumeo && \
    useradd -u 10000 -g kumeo -s /sbin/nologin -M kumeo

# Create app directory and set ownership
WORKDIR /app
COPY --chown=kumeo:kumeo . /app/

# Build the application with LALRPOP
RUN cargo build --release

# Stage 2: Runtime environment using distroless
FROM gcr.io/distroless/cc-debian12:nonroot

# Copy the compiled binary
COPY --from=builder /app/target/release/kumeo-compiler /usr/local/bin/

# Copy required runtime files (if any)
COPY --from=builder /app/examples /opt/kumeo/examples

# Use the nonroot user (uid 65532) from distroless
USER nonroot:nonroot

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/kumeo-compiler"]
CMD ["--help"]