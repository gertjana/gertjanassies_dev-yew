# Page Stats Server

A lightweight Rust web server that provides an HTTP API for page statistics backed by Redis.

## Features

- **RESTful API** for page statistics (views, reads, likes, time)
- **Redis backend** with configurable environment prefixes
- **CORS enabled** for frontend integration
- **Health check endpoint** for monitoring
- **Structured logging** with tracing

## API Endpoints

### Get Page Stats
```
GET /api/stats/{slug}?track_view=true
```
- Returns page statistics for the given slug
- If `track_view=true`, increments view count automatically
- Returns 200 with PageStats JSON

### Increment Statistics
```
POST /api/stats/{slug}/increment
Content-Type: application/json

{
  "increment_type": "views|likes"
}
```

### Add Time Spent
```
POST /api/stats/{slug}/time
Content-Type: application/json

{
  "seconds": 30
}
```

### Get All Stats (Analytics)
```
GET /api/stats
```
- Returns array of all page statistics

### Health Check
```
GET /health
```
- Returns "OK" if server is running

## Configuration

Environment variables:

```bash
REDIS_URL=redis://127.0.0.1:6379  # Redis connection URL
APP_ENV=dev                       # Environment prefix for Redis keys
PORT=3001                         # Server port
HOST=127.0.0.1                    # Server host
```

Or use command line arguments:
```bash
page-stats-server --redis-url redis://localhost:6379 --app-env prod --port 3001
```

## Running

### Development
```bash
cd server
cargo run
```

### Production
```bash
cd server
cargo build --release
./target/release/page-stats-server
```

### With Docker
```bash
cd server
docker build -t page-stats-server .
docker run -p 3001:3001 -e REDIS_URL=redis://host.docker.internal:6379 page-stats-server
```

## Redis Key Format

Keys are stored as: `{APP_ENV}:post:{slug}:page_stats`

Examples:
- `dev:post:240125_rust_on_esp32_2_hardware:page_stats`
- `prod:post:my_blog_post:page_stats`

## Integration with Frontend

The WASM frontend makes HTTP requests to this server:

```javascript
// Get stats with view tracking
fetch('http://localhost:3001/api/stats/my-post?track_view=true')

// Add time spent
fetch('http://localhost:3001/api/stats/my-post/time', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ seconds: 30 })
})
```

## Testing

```bash
cd server
cargo test
```

## Logging

Set log level with `RUST_LOG` environment variable:
```bash
RUST_LOG=info cargo run
RUST_LOG=debug cargo run
```
