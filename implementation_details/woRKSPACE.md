# Cargo Workspace Commands

This project uses a Cargo workspace with the following structure:

```
├── Cargo.toml              # Workspace root
├── frontend/               # Yew frontend application
│   ├── Cargo.toml
│   ├── Trunk.toml
│   ├── src/
│   └── static/
└── server/                 # Axum backend server
    ├── Cargo.toml
    └── src/
```

## Common Commands

### Building

```bash
# Build everything in the workspace
cargo build

# Build specific packages
cargo build -p gertjanassies_dev-yew    # Frontend
cargo build -p page-stats-server        # Server

# Release builds
cargo build --release
cargo build --release -p page-stats-server
```

### Testing

```bash
# Run all tests
cargo test

# Test specific packages
cargo test -p gertjanassies_dev-yew
cargo test -p page-stats-server
```

### Frontend Development

```bash
# Build frontend with Trunk (from frontend directory)
cd frontend
trunk build --release

# Serve frontend for development
cd frontend
trunk serve --port 8080
```

### Server Development

```bash
# Run the server
cargo run -p page-stats-server

# Run with specific arguments
cargo run -p page-stats-server -- --redis-url redis://localhost:6379 --port 3001
```

### Docker

```bash
# Build Docker image (includes both frontend and backend)
docker build -f deploy/Dockerfile -t gertjanassies-combined .

# Run with Docker Compose (includes Redis)
docker-compose up --build
```

## Environment Variables

The server accepts the following environment variables:

- `REDIS_URL`: Redis connection URL (default: `redis://127.0.0.1:6379`)
- `APP_ENV`: Environment prefix for Redis keys (default: `dev`)
- `PORT`: Server port (default: `3001`)
- `HOST`: Server host (default: `127.0.0.1`)
