use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client, RedisResult};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PageStats {
    pub slug: String,
    pub reads: u64,
    pub views: u64,
    pub likes: u64,
    pub time: u64,
}

impl PageStats {
    pub fn new(slug: &str) -> Self {
        Self {
            slug: slug.to_string(),
            reads: 0,
            views: 0,
            likes: 0,
            time: 0,
        }
    }

    pub fn increment_views(&mut self) {
        self.views += 1;
    }

    pub fn increment_likes(&mut self) {
        self.likes += 1;
    }

    pub fn add_time(&mut self, seconds: u64) {
        self.time += seconds;
    }
}

pub struct RedisPageStatsClient {
    connection_manager: ConnectionManager,
    env_prefix: String,
}

impl RedisPageStatsClient {
    /// Create a new Redis client for page stats
    ///
    /// # Arguments
    /// * `redis_url` - Redis connection URL (e.g., "redis://127.0.0.1:6379")
    /// * `env_prefix` - Environment prefix for keys (e.g., "prod", "dev", "staging")
    pub async fn new(redis_url: &str, env_prefix: &str) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        let connection_manager = ConnectionManager::new(client).await?;
        Ok(Self {
            connection_manager,
            env_prefix: env_prefix.to_string(),
        })
    }

    /// Create a Redis client using environment variables
    ///
    /// Expected environment variables:
    /// - REDIS_URL: Redis connection URL
    /// - APP_ENV: Environment prefix (defaults to "dev")
    pub async fn _from_env() -> RedisResult<Self> {
        let redis_url =
            env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let env_prefix = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());

        Self::new(&redis_url, &env_prefix).await
    }

    /// Generate the Redis key for a given slug
    /// Format: <env>:post:<slug>:page_stats
    fn generate_key(&self, slug: &str) -> String {
        format!("{}:post:{}:page_stats", self.env_prefix, slug)
    }

    /// Get a clone of the connection manager
    fn get_connection(&self) -> ConnectionManager {
        self.connection_manager.clone()
    }

    /// Get page stats for a specific slug
    /// Returns None if the key doesn't exist
    pub async fn get_page_stats(&self, slug: &str) -> RedisResult<Option<PageStats>> {
        let mut conn = self.get_connection();
        let key = self.generate_key(slug);

        let json_string: Option<String> = conn.get(&key).await?;

        match json_string {
            Some(json) => {
                match serde_json::from_str::<PageStats>(&json) {
                    Ok(stats) => Ok(Some(stats)),
                    Err(_) => {
                        // If JSON parsing fails, return a new PageStats with the slug
                        Ok(Some(PageStats::new(slug)))
                    }
                }
            }
            None => Ok(None),
        }
    }

    /// Set page stats for a specific slug
    pub async fn set_page_stats(&self, stats: &PageStats) -> RedisResult<()> {
        let mut conn = self.get_connection();
        let key = self.generate_key(&stats.slug);

        let json_string = serde_json::to_string(stats).map_err(|e| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "JSON serialization failed",
                e.to_string(),
            ))
        })?;

        conn.set::<_, _, ()>(&key, json_string).await?;
        Ok(())
    }

    /// Increment the view count for a specific slug
    pub async fn increment_views(&self, slug: &str) -> RedisResult<PageStats> {
        let mut stats = self
            .get_page_stats(slug)
            .await?
            .unwrap_or_else(|| PageStats::new(slug));

        stats.increment_views();
        self.set_page_stats(&stats).await?;
        Ok(stats)
    }

    /// Increment the like count for a specific slug
    pub async fn increment_likes(&self, slug: &str) -> RedisResult<PageStats> {
        let mut stats = self
            .get_page_stats(slug)
            .await?
            .unwrap_or_else(|| PageStats::new(slug));

        stats.increment_likes();
        self.set_page_stats(&stats).await?;
        Ok(stats)
    }

    /// Add time spent on a page for a specific slug
    pub async fn add_time(&self, slug: &str, seconds: u64) -> RedisResult<PageStats> {
        let mut stats = self
            .get_page_stats(slug)
            .await?
            .unwrap_or_else(|| PageStats::new(slug));

        stats.add_time(seconds);
        self.set_page_stats(&stats).await?;
        Ok(stats)
    }

    /// Get all page stats (useful for analytics)
    /// Returns a vector of all PageStats found in Redis
    pub async fn get_all_page_stats(&self) -> RedisResult<Vec<PageStats>> {
        let mut conn = self.get_connection();
        let pattern = format!("{}:post:*:page_stats", self.env_prefix);

        let mut all_stats = Vec::new();
        let mut cursor = 0u64;

        loop {
            let (new_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .cursor_arg(cursor)
                .arg("MATCH")
                .arg(&pattern)
                .query_async(&mut conn)
                .await?;

            for key in keys {
                if let Ok(Some(json_string)) = conn.get::<_, Option<String>>(&key).await {
                    if let Ok(stats) = serde_json::from_str::<PageStats>(&json_string) {
                        all_stats.push(stats);
                    }
                }
            }

            cursor = new_cursor;
            if cursor == 0 {
                break;
            }
        }

        Ok(all_stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_page_stats_creation() {
        let stats = PageStats::new("test_slug");
        assert_eq!(stats.slug, "test_slug");
        assert_eq!(stats.views, 0);
        assert_eq!(stats.reads, 0);
        assert_eq!(stats.likes, 0);
        assert_eq!(stats.time, 0);
    }

    #[tokio::test]
    async fn test_page_stats_increments() {
        let mut stats = PageStats::new("test_slug");

        stats.increment_views();
        assert_eq!(stats.views, 1);

        stats.increment_likes();
        assert_eq!(stats.likes, 1);

        stats.add_time(30);
        assert_eq!(stats.time, 30);
    }

    #[tokio::test]
    async fn test_key_generation() {
        let client = RedisPageStatsClient::new("redis://localhost", "test")
            .await
            .unwrap();
        let key = client.generate_key("my_blog_post");
        assert_eq!(key, "test:post:my_blog_post:page_stats");
    }

    #[test]
    fn test_json_serialization() {
        let stats = PageStats {
            slug: "240125_rust_on_esp32_2_hardware".to_string(),
            reads: 0,
            views: 2,
            likes: 0,
            time: 6,
        };

        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: PageStats = serde_json::from_str(&json).unwrap();

        assert_eq!(stats, deserialized);
    }
}
