use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use clap::Parser;
use serde::Deserialize;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

mod redis_client;
use redis_client::{PageStats, RedisPageStatsClient};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Redis URL
    #[arg(long, env = "REDIS_URL", default_value = "redis://127.0.0.1:6379")]
    redis_url: String,

    /// Environment prefix for Redis keys
    #[arg(long, env = "APP_ENV", default_value = "dev")]
    app_env: String,

    /// Port to listen on
    #[arg(short, long, env = "PORT", default_value = "3001")]
    port: u16,

    /// Host to bind to
    #[arg(long, env = "HOST", default_value = "127.0.0.1")]
    host: String,
}

#[derive(Clone)]
struct AppState {
    redis_client: Arc<RedisPageStatsClient>,
}

#[derive(Deserialize)]
struct StatsQuery {
    track_view: Option<bool>,
}

#[derive(Deserialize)]
struct IncrementRequest {
    increment_type: String, // "views", "likes"
    _amount: Option<u64>,
}

#[derive(Deserialize)]
struct TimeRequest {
    seconds: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    info!("Starting page stats server...");
    info!("Redis URL: {}", args.redis_url);
    info!("Environment: {}", args.app_env);
    info!("Server: {}:{}", args.host, args.port);

    // Initialize Redis client
    let redis_client = Arc::new(
        RedisPageStatsClient::new(&args.redis_url, &args.app_env)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to connect to Redis: {}", e))?,
    );

    let app_state = AppState { redis_client };

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/stats/:slug", get(get_page_stats))
        .route("/api/stats/:slug/increment", post(increment_stats))
        .route("/api/stats/:slug/reading-time", post(set_reading_time))
        .route("/api/stats", get(get_all_stats))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", args.host, args.port)).await?;
    info!("Server listening on {}:{}", args.host, args.port);

    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

/// Get page stats for a specific slug
async fn get_page_stats(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<StatsQuery>,
) -> Result<Json<PageStats>, StatusCode> {
    info!(
        "Getting stats for slug: {} (track_view: {:?})",
        slug, query.track_view
    );

    let track_view = query.track_view.unwrap_or(false);

    let stats = if track_view {
        // Increment view count and return updated stats
        match state.redis_client.increment_views(&slug).await {
            Ok(stats) => stats,
            Err(e) => {
                warn!("Failed to increment views for {}: {}", slug, e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    } else {
        // Just get existing stats
        match state.redis_client.get_page_stats(&slug).await {
            Ok(Some(stats)) => stats,
            Ok(None) => PageStats::new(&slug),
            Err(e) => {
                warn!("Failed to get stats for {}: {}", slug, e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    };

    Ok(Json(stats))
}

/// Increment specific stat types
async fn increment_stats(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(payload): Json<IncrementRequest>,
) -> Result<Json<PageStats>, StatusCode> {
    info!("Incrementing {} for slug: {}", payload.increment_type, slug);

    let stats = match payload.increment_type.as_str() {
        "views" => state.redis_client.increment_views(&slug).await,
        "likes" => state.redis_client.increment_likes(&slug).await,
        _ => {
            warn!("Invalid increment type: {}", payload.increment_type);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    match stats {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => {
            warn!(
                "Failed to increment {} for {}: {}",
                payload.increment_type, slug, e
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Set reading time for a page (only if not already set)
async fn set_reading_time(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(payload): Json<TimeRequest>,
) -> Result<Json<PageStats>, StatusCode> {
    info!(
        "Setting reading time {} seconds for slug: {}",
        payload.seconds, slug
    );

    match state
        .redis_client
        .set_reading_time(&slug, payload.seconds)
        .await
    {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => {
            warn!("Failed to set reading time for {}: {}", slug, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get all page stats (for analytics)
async fn get_all_stats(State(state): State<AppState>) -> Result<Json<Vec<PageStats>>, StatusCode> {
    info!("Getting all page stats");

    match state.redis_client.get_all_page_stats().await {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => {
            warn!("Failed to get all stats: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
