# Redis Page Stats Client

A Redis client module for tracking page statistics in the blog. This module provides functionality to read and write page statistics stored in Redis with the key format `<env>:post:<slug>:page_stats`.

## Data Structure

The page stats are stored as JSON with the following structure:

```json
{
  "slug": "240125_rust_on_esp32_2_hardware",
  "reads": 0,
  "views": 2,
  "likes": 0,
  "time": 6
}
```

## Features

- **Track page views**: Increment view count when a page is loaded
- **Track page reads**: Increment read count when a page is fully read
- **Track time spent**: Add time spent on each page
- **Track likes**: Increment like count when users like a post
- **Analytics**: Get all page stats for analytics purposes
- **Environment-aware**: Uses environment prefixes for different deployments

## Usage

### Environment Variables

Set these environment variables:

```bash
export REDIS_URL="redis://localhost:6379"  # Redis connection URL
export APP_ENV="dev"                       # Environment prefix (dev, staging, prod)
```

### Basic Usage

```rust
use crate::redis_client::{RedisPageStatsClient, PageStats};

// Create client from environment variables
let client = RedisPageStatsClient::from_env()?;

// Or create with explicit parameters
let client = RedisPageStatsClient::new("redis://localhost:6379", "dev")?;

// Track a page view
let stats = client.increment_views("my_blog_post").await?;

// Track a page read completion
let stats = client.increment_reads("my_blog_post").await?;

// Add time spent on page (in seconds)
let stats = client.add_time("my_blog_post", 30).await?;

// Track a like
let stats = client.increment_likes("my_blog_post").await?;

// Get current stats
let stats = client.get_page_stats("my_blog_post").await?;

// Check if stats exist
let exists = client.exists("my_blog_post").await?;

// Get all page stats (for analytics)
let all_stats = client.get_all_page_stats().await?;
```

### Integration with Yew Components

For easy integration with Yew components, use the helper functions in `page_stats_examples.rs`:

```rust
use crate::page_stats_examples::{track_page_view, track_page_read, track_time_spent, track_like};

// In your Yew component
#[function_component(BlogPost)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    let slug = props.slug.clone();

    // Track page view when component mounts
    use_effect_with(slug.clone(), move |slug| {
        let slug = slug.clone();
        spawn_local(async move {
            if let Ok(_) = track_page_view(&slug).await {
                console::log_1(&"Page view tracked".into());
            }
        });
        || ()
    });

    // ... rest of your component
}
```

### Custom Stats

You can also create and set custom page stats:

```rust
let custom_stats = PageStats {
    slug: "my_blog_post".to_string(),
    reads: 10,
    views: 25,
    likes: 3,
    time: 120,
};

client.set_page_stats(&custom_stats).await?;
```

## Redis Key Format

The Redis keys follow this pattern:
```
<env>:post:<slug>:page_stats
```

Examples:
- `dev:post:240125_rust_on_esp32_2_hardware:page_stats`
- `prod:post:my_blog_post:page_stats`
- `staging:post:another_post:page_stats`

## API Reference

### `PageStats`

```rust
pub struct PageStats {
    pub slug: String,
    pub reads: u64,
    pub views: u64,
    pub likes: u64,
    pub time: u64,
}
```

Methods:
- `new(slug: &str)` - Create new stats with zero values
- `increment_views()` - Increment view count
- `increment_reads()` - Increment read count
- `increment_likes()` - Increment like count
- `add_time(seconds: u64)` - Add time spent

### `RedisPageStatsClient`

Constructor methods:
- `new(redis_url: &str, env_prefix: &str)` - Create with explicit parameters
- `from_env()` - Create using environment variables

Page stats methods:
- `get_page_stats(slug: &str)` - Get stats for a slug
- `set_page_stats(stats: &PageStats)` - Set stats for a slug
- `increment_views(slug: &str)` - Increment views and return updated stats
- `increment_reads(slug: &str)` - Increment reads and return updated stats
- `increment_likes(slug: &str)` - Increment likes and return updated stats
- `add_time(slug: &str, seconds: u64)` - Add time and return updated stats

Utility methods:
- `exists(slug: &str)` - Check if stats exist
- `delete_page_stats(slug: &str)` - Delete stats for a slug
- `get_all_page_stats()` - Get all page stats for analytics

## Testing

Run the tests with:

```bash
cargo test redis_client
```

## Dependencies

- `redis` - Redis client for Rust
- `serde` - Serialization framework
- `serde_json` - JSON support for serde
- `tokio` - Async runtime (with macros and rt-multi-thread features)
