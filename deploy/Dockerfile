# Frontend build stage using Rust 1.90 with wasm target
FROM rust:1.90-alpine AS frontend-builder

# Install build dependencies for Alpine and frontend tooling
RUN apk add --no-cache musl-dev nodejs npm curl

# Install Sass via npm to avoid glibc compatibility issues
RUN npm install -g sass

# Install trunk and wasm-bindgen for building the frontend
RUN cargo install --locked trunk
RUN cargo install --locked wasm-bindgen-cli

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy root Cargo files first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Copy both frontend and backend directories (trunk needs the workspace context)
COPY frontend ./frontend
COPY backend ./backend

# Build the frontend (trunk will use the Trunk.toml config in frontend/)
WORKDIR /app/frontend
RUN trunk build --release

# Backend build stage using Rust 1.90 for compatibility
FROM rust:1.90-alpine AS backend-builder

# Install build dependencies for Alpine
RUN apk add --no-cache musl-dev

WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml ./backend/
COPY frontend/Cargo.toml ./frontend/

# Create dummy source files to build dependencies
RUN mkdir -p backend/src frontend/src && \
    echo "fn main() {}" > backend/src/main.rs && \
    echo "fn main() {}" > frontend/src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release --package page-stats-server

# Copy actual source code
COPY backend/src ./backend/src
COPY frontend/src ./frontend/src

# Build the actual application
RUN cargo build --release --package page-stats-server

# Final runtime stage
FROM nginx:alpine

# Install supervisor to manage multiple processes
RUN apk add --no-cache supervisor

# Install required packages for the Rust server
RUN apk add --no-cache ca-certificates

# Copy the nginx configuration
COPY deploy/nginx.conf /etc/nginx/conf.d/default.conf

# Copy the built frontend from the frontend-builder stage
COPY --from=frontend-builder /app/frontend/dist /usr/share/nginx/html

# Copy the Rust server binary from the backend-builder stage
COPY --from=backend-builder /app/target/release/page-stats-server /usr/local/bin/page-stats-server

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
