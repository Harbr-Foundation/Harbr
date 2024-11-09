# Build stage
FROM rust:1.82-alpine AS builder

# Use the ARG in a RUN command to force cache invalidation
RUN echo "Cache bust: ${CACHE_BUST}"

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev mysql-dev git

# Create non-root user for builds
RUN addgroup -S rust && \
    adduser -S rust -G rust && \
    mkdir -p /usr/src/app && \
    chown -R rust:rust /usr/src/app

# Set working directory and switch to rust user
WORKDIR /usr/src/app

# Give rust user ownership of cargo home
ENV CARGO_HOME=/usr/local/cargo
RUN mkdir -p ${CARGO_HOME} && \
    chown -R rust:rust ${CARGO_HOME}

COPY . ./Harbr
RUN chown -R rust:rust /usr/src/app/Harbr

USER rust

# Set up cargo
WORKDIR /usr/src/app/Harbr
RUN cargo --version && \
    ls -la && \
    # Initialize new cargo project if needed
    if [ ! -f Cargo.toml ]; then \
        cargo init; \
    fi

# Build for release
RUN cargo build --release

# Final stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache libgcc openssl mysql-client

WORKDIR /usr/src/app

# Copy the built executable from builder
COPY --from=builder /usr/src/app/Harbr/target/release/harbr ./harbr
COPY --from=builder /usr/src/app/Harbr/harbr.config.json ./harbr.config.json
COPY --from=builder /usr/src/app/Harbr/repo.json ./repo.json
COPY --from=builder /usr/src/app/Harbr/shortcuts.json ./shortcuts.json
COPY --from=builder /usr/src/app/Harbr/activity.json ./activity.json

# Create a non-root user
RUN adduser -D appuser && \
    chown appuser:appuser harbr

USER appuser

# Expose the API port
EXPOSE 8080

# Run the API
CMD ["./harbr", "run"]