# Multi-stage build for Rust backend using Alpine for compatibility
FROM rust:1.82-alpine AS rust-builder

# Install build dependencies for Alpine
RUN apk add --no-cache musl-dev

WORKDIR /app/server
COPY server/Cargo.toml server/Cargo.lock ./
COPY server/src ./src

RUN cargo build --release

# Build stage for frontend (if needed)
FROM nginx:alpine

# Install supervisor to manage multiple processes
RUN apk add --no-cache supervisor

# Install required packages for the Rust server
RUN apk add --no-cache ca-certificates

# Copy the nginx configuration
COPY deploy/nginx.conf /etc/nginx/conf.d/default.conf

# Copy the built application from the dist directory
COPY dist /usr/share/nginx/html

# Copy the Rust server binary
COPY --from=rust-builder /app/server/target/release/page-stats-server /usr/local/bin/page-stats-server

# Create supervisor configuration and directories
RUN mkdir -p /etc/supervisor/conf.d /var/log/supervisor /var/run
COPY deploy/supervisord.conf /etc/supervisord.conf

# Set default environment variables for the page stats server
ENV REDIS_URL=redis://127.0.0.1:6379
ENV APP_ENV=production
ENV PAGE_STATS_PORT=3001
ENV PAGE_STATS_HOST=127.0.0.1

# Expose port 80
EXPOSE 80

# Start supervisor (which will manage nginx and the Rust server)
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
