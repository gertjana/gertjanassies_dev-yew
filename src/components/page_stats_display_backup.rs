use serde::{Deserialize, Serialize};
use serde_json;
use serde_wasm_bindgen;
use std::error::Error;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PageStats {
    pub slug: String,
    pub reads: u64,
    pub views: u64,
    pub likes: u64,
    pub time: u64,
}

#[derive(Properties, PartialEq)]
pub struct PageStatsDisplayProps {
    pub slug: AttrValue,
    #[prop_or(false)]
    pub track_view: bool,
    #[prop_or(0)]
    pub reading_time_seconds: u32,
}

#[function_component(PageStatsDisplay)]
pub fn page_stats_display(props: &PageStatsDisplayProps) -> Html {
    let stats = use_state(|| None::<PageStats>);
    let loading = use_state(|| true);
    let error = use_state(|| false);

    let slug = props.slug.clone();
    let track_view = props.track_view;
    let reading_time_seconds = props.reading_time_seconds;

    // Load and optionally track view on component mount
    {
        let stats = stats.clone();
        let loading = loading.clone();
        let error = error.clone();
        let slug = slug.clone();

        use_effect_with(
            (slug.clone(), track_view, reading_time_seconds),
            move |(slug, track_view, reading_time)| {
                let stats = stats.clone();
                let loading = loading.clone();
                let error = error.clone();
                let slug = slug.clone();
                let track_view = *track_view;
                let reading_time = *reading_time;

                spawn_local(async move {
                    match load_page_stats_from_server(&slug, track_view, reading_time).await {
                        Ok(page_stats) => {
                            stats.set(Some(page_stats));
                            loading.set(false);
                        }
                        Err(err) => {
                            console::error_1(&format!("Failed to load page stats: {}", err).into());
                            error.set(true);
                            loading.set(false);
                        }
                    }
                });

                || ()
            },
        );
    }

    if *loading {
        return html! {
            <div class="page-stats">
                <span class="stats-loading">{"Loading stats..."}</span>
            </div>
        };
    }

    if *error {
        return html! {
            <div class="page-stats">
                <span class="stats-error">{"Stats unavailable"}</span>
            </div>
        };
    }

    match stats.as_ref() {
        Some(page_stats) => {
            let slug = props.slug.clone();
            let stats_clone = stats.clone();

            let on_like = {
                let slug = slug.clone();
                let stats = stats_clone.clone();
                Callback::from(move |_| {
                    let slug = slug.clone();
                    let stats = stats.clone();
                    spawn_local(async move {
                        if let Err(e) = increment_stat(&slug, "likes").await {
                            console::error_1(&format!("Failed to increment likes: {}", e).into());
                        } else {
                            // Reload stats to get updated counts
                            if let Ok(updated_stats) =
                                load_page_stats_from_server(&slug, false, 0).await
                            {
                                stats.set(Some(updated_stats));
                            }
                        }
                    });
                })
            };

            html! {
                <div class="page-stats">
                    <span class="stat-item">{page_stats.views}{" views"}</span>
                    <span class="stat-separator">{" • "}</span>
                    <span class="stat-item">{format_time(page_stats.time)}{" read"}</span>
                    <span class="stat-separator">{" • "}</span>
                    <button class="like-button" onclick={on_like}>
                        <span class="like-count">{page_stats.likes}</span>
                        <span class="thumbs-up">{"�"}</span>
                    </button>
                </div>
            }
        }
        None => {
            html! {
                <div class="page-stats">
                    <span class="stats-placeholder">{"No stats available"}</span>
                </div>
            }
        }
    }
}

// Helper function to format time in a human-readable way
fn format_time(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;
        if remaining_seconds == 0 {
            format!("{}m", minutes)
        } else {
            format!("{}m {}s", minutes, remaining_seconds)
        }
    } else {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        if minutes == 0 {
            format!("{}h", hours)
        } else {
            format!("{}h {}m", hours, minutes)
        }
    }
}

// Increment a specific stat type
async fn increment_stat(slug: &str, stat_type: &str) -> Result<PageStats, Box<dyn Error>> {
    let window = web_sys::window().unwrap();

    let increment_url = format!("/api/stats/{}/increment", slug);

    // Create JSON payload
    let payload = serde_json::json!({
        "increment_type": stat_type
    });

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::SameOrigin);

    // Set content type header
    let headers = web_sys::Headers::new().unwrap();
    headers.set("Content-Type", "application/json").unwrap();
    opts.set_headers(&headers);

    // Set body
    opts.set_body(&wasm_bindgen::JsValue::from_str(&payload.to_string()));

    let request = Request::new_with_str_and_init(&increment_url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Network error: {:?}", e))?;

    let resp: Response = resp_value.dyn_into().unwrap();

    if resp.ok() {
        let json = JsFuture::from(resp.json().unwrap())
            .await
            .map_err(|e| format!("Failed to parse JSON: {:?}", e))?;

        serde_wasm_bindgen::from_value::<PageStats>(json).map_err(|e| -> Box<dyn Error> {
            format!("Failed to deserialize stats: {:?}", e).into()
        })
    } else {
        Err(format!("Failed to increment {}: HTTP {}", stat_type, resp.status()).into())
    }
}

// Load page stats from the Rust server API
async fn load_page_stats_from_server(
    slug: &str,
    track_view: bool,
    reading_time_seconds: u32,
) -> Result<PageStats, Box<dyn Error>> {
    let window = web_sys::window().unwrap();

    // First, get stats (with optional view tracking)
    let get_url = if track_view {
        format!("/api/stats/{}?track_view=true", slug)
    } else {
        format!("/api/stats/{}", slug)
    };

    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);

    let request = Request::new_with_str_and_init(&get_url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Network error: {:?}", e))?;

    let resp: Response = resp_value.dyn_into().unwrap();

    let mut stats = if resp.ok() {
        let json = JsFuture::from(resp.json().unwrap())
            .await
            .map_err(|e| format!("Failed to parse JSON: {:?}", e))?;

        serde_wasm_bindgen::from_value::<PageStats>(json)
            .map_err(|e| format!("Failed to deserialize stats: {:?}", e))?
    } else {
        console::error_1(&format!("Failed to load stats: HTTP {}", resp.status()).into());
        PageStats {
            slug: slug.to_string(),
            views: 0,
            reads: 0,
            likes: 0,
            time: 0,
        }
    };

    // Use the calculated reading time instead of tracked time
    stats.time = reading_time_seconds as u64;

    console::log_1(
        &format!(
            "Loaded stats for '{}': {} views, {} reads, {} likes, {} seconds reading time",
            slug, stats.views, stats.reads, stats.likes, reading_time_seconds
        )
        .into(),
    );

    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(30), "30s");
        assert_eq!(format_time(60), "1m");
        assert_eq!(format_time(90), "1m 30s");
        assert_eq!(format_time(3600), "1h");
        assert_eq!(format_time(3660), "1h 1m");
        assert_eq!(format_time(3690), "1h 1m");
    }
}
