# Environment Variables Configuration

The combined Docker container supports the following environment variables to configure the Redis backend server:

## Redis Configuration

### `REDIS_URL`
- **Description**: Full Redis connection URL
- **Default**: `redis://127.0.0.1:6379`
- **Examples**:
  - Local Redis: `redis://127.0.0.1:6379`
  - Remote Redis: `redis://my-redis-host:6379`
  - Redis with auth: `redis://user:password@redis-host:6379`
  - Redis with SSL: `rediss://user:password@redis-host:6380`

### `APP_ENV`
- **Description**: Environment prefix for Redis keys (used to separate data between environments)
- **Default**: `production`
- **Examples**: `dev`, `staging`, `production`, `test`

## Server Configuration

### `PAGE_STATS_PORT`
- **Description**: Port for the internal page stats server
- **Default**: `3001`
- **Note**: This is internal to the container, nginx reverse proxy handles external requests

### `PAGE_STATS_HOST`
- **Description**: Host/IP for the internal page stats server to bind to
- **Default**: `127.0.0.1`
- **Note**: Should remain `127.0.0.1` for container security

## Usage Examples

### Basic Usage (Default Redis)
```bash
docker run -p 80:80 gertjanassies-combined
```

### With External Redis Server
```bash
docker run -p 80:80 \
  -e REDIS_URL=redis://my-redis-server:6379 \
  -e APP_ENV=production \
  gertjanassies-combined
```

### With Redis Authentication
```bash
docker run -p 80:80 \
  -e REDIS_URL=redis://username:password@redis-server:6379 \
  -e APP_ENV=production \
  gertjanassies-combined
```

### For Render.com Deployment
Set these environment variables in your Render.com service configuration:
- `REDIS_URL`: Your Redis connection string (e.g., from Render Redis or external provider)
- `APP_ENV`: `production` (or `staging` for staging deployments)

## Docker Compose Example

```yaml
version: '3.8'
services:
  redis:
    image: redis:alpine
    ports:
      - "6379:6379"

  blog:
    image: gertjanassies-combined
    ports:
      - "80:80"
    environment:
      - REDIS_URL=redis://redis:6379
      - APP_ENV=production
    depends_on:
      - redis
```

## Health Check

The container includes a health check endpoint at `/health` that returns `200 OK` regardless of Redis connectivity. This is useful for container orchestration platforms like Render.com.

## Redis Data Structure

The application stores page statistics in Redis with keys prefixed by `APP_ENV`:
- `{APP_ENV}:stats:{slug}` - Contains JSON with views, reads, likes, and time data

Example with `APP_ENV=production`:
- `production:stats:home` - Stats for the home page
- `production:stats:blog-post-slug` - Stats for a blog post
