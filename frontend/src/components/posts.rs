use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window};
use yew::prelude::*;
use yew_router::prelude::*;

use super::markdown::{load_markdown_content, render_markdown_to_html};
use super::page_stats_display::PageStatsDisplay;
use crate::app::Route;
use crate::reading_time::calculate_reading_time;

#[allow(dead_code)]
#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub slug: String,
}

#[allow(dead_code)]
#[derive(Properties, PartialEq)]
pub struct PostsProps {
    #[prop_or(false)]
    pub featured_only: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PostFrontmatter {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub published: bool,
    #[serde(default)]
    pub image: String,
}

impl Default for PostFrontmatter {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            date: "".to_string(),
            summary: "".to_string(),
            author: "".to_string(),
            tags: vec![],
            category: "".to_string(),
            published: true,
            image: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostSummary {
    pub slug: String,
    pub frontmatter: PostFrontmatter,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Post {
    pub slug: String,
    pub frontmatter: PostFrontmatter,
    pub content: String, // Raw markdown content
}

fn update_url_with_filter(key: &str, value: &str) {
    if let Some(window) = window() {
        // Always navigate to /blog when filtering, regardless of current page
        let new_url = format!("/blog?{}={}", key, value);

        if let Ok(history) = window.history() {
            let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));
            let _ = window.location().reload();
        }
    }
}

#[function_component(Posts)]
pub fn posts(props: &PostsProps) -> Html {
    let posts_data = use_state(Vec::<PostSummary>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let posts_data = posts_data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            let posts_data = posts_data.clone();
            let loading = loading.clone();
            let error = error.clone();

            spawn_local(async move {
                match load_all_posts().await {
                    Ok(all_posts) => {
                        posts_data.set(all_posts);
                        loading.set(false);
                    }
                    Err(err) => {
                        error.set(Some(err));
                        loading.set(false);
                    }
                }
            });

            || ()
        });
    }

    if *loading {
        return html! {
            <div class="posts-container">
                <div class="loading">{ "Loading posts..." }</div>
            </div>
        };
    }

    if let Some(err) = (*error).as_ref() {
        return html! {
            <div class="posts-container">
                <div class="error">{ format!("Error loading posts: {}", err) }</div>
            </div>
        };
    }

    let filters = get_url_filters();

    let has_filters =
        filters.featured.is_some() || filters.category.is_some() || filters.tag.is_some();

    let mut filtered_posts: Vec<&PostSummary> = posts_data
        .iter()
        .filter(|post| {
            let show_drafts = filters.draft.unwrap_or(false);
            if !show_drafts && !post.frontmatter.published {
                return false; // Hide drafts by default
            }

            if props.featured_only {
                return post.frontmatter.tags.contains(&"featured".to_string());
            }

            if !has_filters {
                return true;
            }

            if let Some(required_featured) = filters.featured {
                let has_featured_tag = post.frontmatter.tags.contains(&"featured".to_string());
                if has_featured_tag != required_featured {
                    return false;
                }
            }

            if let Some(ref required_category) = filters.category {
                let post_categories: Vec<&str> = post
                    .frontmatter
                    .category
                    .split(',')
                    .map(|cat| cat.trim())
                    .collect();
                if !post_categories.contains(&required_category.as_str()) {
                    return false;
                }
            }

            if let Some(ref required_tag) = filters.tag {
                if !post.frontmatter.tags.contains(required_tag) {
                    return false;
                }
            }

            true
        })
        .collect();

    filtered_posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

    html! {
        <div class="posts-container">
            <div class="posts-list">
                { for filtered_posts.iter().map(|post| {
                    html! {
                        <PostListItem post={(*post).clone()} />
                    }
                }) }
            </div>

            if filtered_posts.is_empty() {
                <div class="no-posts">
                    <p>{ "No posts found matching the current filters." }</p>
                </div>
            }
        </div>
    }
}

#[allow(dead_code)]
#[derive(Properties, PartialEq)]
pub struct PostCardProps {
    pub post: PostSummary,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    let post = &props.post;

    html! {
        <article class="post-card">
            if !post.frontmatter.image.is_empty() {
                <div class="post-image">
                    <img src={post.frontmatter.image.clone()} alt={post.frontmatter.title.clone()} />
                </div>
            }

            <div class="post-card-content">
                <h2 class="post-title">
                    <Link<Route> to={Route::Post { slug: post.slug.clone() }}>{ &post.frontmatter.title }</Link<Route>>
                </h2>

                <div class="post-meta">
                    if !post.frontmatter.date.is_empty() {
                        <time>{ &post.frontmatter.date }</time>
                    }
                    if !post.frontmatter.author.is_empty() {
                        <span class="author">{ format!("By {}", &post.frontmatter.author) }</span>
                    }
                    if !post.frontmatter.category.is_empty() {
                        <span class="category">{ &post.frontmatter.category }</span>
                    }
                </div>

                if !post.frontmatter.summary.is_empty() {
                    <p class="post-summary">{ &post.frontmatter.summary }</p>
                }

                if !post.frontmatter.tags.is_empty() {
                    <div class="post-tags">
                        { for post.frontmatter.tags.iter().take(3).map(|tag| {
                            html! { <span class="tag">{ tag }</span> }
                        })}
                        if post.frontmatter.tags.len() > 3 {
                            <span class="tag more">{ format!("+{}", post.frontmatter.tags.len() - 3) }</span>
                        }
                    </div>
                }
            </div>
        </article>
    }
}

#[derive(Properties, PartialEq)]
pub struct PostListItemProps {
    pub post: PostSummary,
}

#[function_component(PostListItem)]
pub fn post_list_item(props: &PostListItemProps) -> Html {
    let post = &props.post;

    html! {
        <article class="post-list-item">
            // Small image on the left with author and date below
            <div class="post-image-section">
                <div class="post-image">
                    if !post.frontmatter.image.is_empty() {
                        <img src={post.frontmatter.image.clone()} alt={post.frontmatter.title.clone()} />
                    } else {
                        <div class="placeholder-image"></div>
                    }
                </div>
                <div class="post-meta">
                    <time class="post-date">{ &post.frontmatter.date }</time>
                    <span class="post-author">{ format!("By {}", &post.frontmatter.author) }</span>
                </div>
            </div>

            <div class="post-content">
                <div class="post-header">
                    <h2 class="post-title">
                        <Link<Route> to={Route::Post { slug: post.slug.clone() }}>{ &post.frontmatter.title }</Link<Route>>
                    </h2>

                    // Categories and tags next to the title
                    <div class="post-taxonomy">
                        // Split categories by comma and show as separate items
                        { for post.frontmatter.category.split(',').map(|cat| cat.trim()).filter(|cat| !cat.is_empty()).map(|category| {
                            let category_str = category.to_string();
                            let onclick = {
                                let category = category_str.clone();
                                Callback::from(move |_: MouseEvent| {
                                    update_url_with_filter("category", &category);
                                })
                            };
                            html! {
                                <span class="category clickable" {onclick}>
                                    { category }
                                </span>
                            }
                        })}

                        { for post.frontmatter.tags.iter().filter(|tag| *tag != "featured").map(|tag| {
                            let tag_str = tag.clone();
                            let onclick = {
                                let tag = tag_str.clone();
                                Callback::from(move |_: MouseEvent| {
                                    update_url_with_filter("tag", &tag);
                                })
                            };
                            html! {
                                <span class="tag clickable" {onclick}>
                                    { tag }
                                </span>
                            }
                        })}
                    </div>
                </div>

                if !post.frontmatter.summary.is_empty() {
                    <p class="post-summary">{ &post.frontmatter.summary }</p>
                }
            </div>
        </article>
    }
}

#[derive(Properties, PartialEq)]
pub struct PostViewProps {
    pub slug: String,
}

#[function_component(PostView)]
pub fn post_view(props: &PostViewProps) -> Html {
    let post_data = use_state(|| None::<Post>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Load the post on component mount
    {
        let post_data = post_data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let slug = props.slug.clone();

        use_effect_with(props.slug.clone(), move |_| {
            let post_data = post_data.clone();
            let loading = loading.clone();
            let error = error.clone();

            // Reset state when slug changes
            post_data.set(None);
            loading.set(true);
            error.set(None);

            spawn_local(async move {
                match load_post(&slug).await {
                    Ok(post) => {
                        post_data.set(Some(post));
                        loading.set(false);
                    }
                    Err(err) => {
                        error.set(Some(err));
                        loading.set(false);
                    }
                }
            });

            || ()
        });
    }

    // Render KaTeX when post content is loaded
    {
        let loading = loading.clone();
        let post_data = post_data.clone();

        use_effect_with((*loading, (*post_data).clone()), move |_| {
            if !*loading && post_data.is_some() {
                // Delay the KaTeX rendering to ensure DOM is updated
                spawn_local(async {
                    wasm_bindgen_futures::JsFuture::from(web_sys::js_sys::Promise::new(
                        &mut |resolve, _| {
                            web_sys::window()
                                .unwrap()
                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                    &resolve, 100,
                                )
                                .unwrap();
                        },
                    ))
                    .await
                    .unwrap();

                    // Call the global renderMath function
                    if web_sys::window().is_some() {
                        let _ =
                            web_sys::js_sys::eval("if (window.renderMath) window.renderMath();");
                    }
                });
            }
            || ()
        });
    }

    if *loading {
        return html! {
            <div class="posts-container">
                <div class="loading">{ "Loading post..." }</div>
            </div>
        };
    }

    if let Some(err) = (*error).as_ref() {
        return html! {
            <div class="posts-container">
                <div class="error">{ format!("Error loading post: {}", err) }</div>
            </div>
        };
    }

    let Some(post) = (*post_data).as_ref() else {
        return html! {
            <div class="posts-container">
                <div class="error">{ "Post not found" }</div>
            </div>
        };
    };

    html! {
        <div class="posts-container">
            <div class="post-view-container">
                // Full width title
                <h1 class="post-view-title">{ &post.frontmatter.title }</h1>

                // Meta information below title
                <div class="post-view-meta">
                    <time class="post-date">{ &post.frontmatter.date }</time>
                    <span class="post-author">{ format!("By {}", &post.frontmatter.author) }</span>
                    if post.frontmatter.tags.contains(&"featured".to_string()) {
                        <span class="featured-badge">{ "Featured" }</span>
                    }
                </div>

                // Taxonomy (categories and tags)
                <div class="post-view-taxonomy">
                    // Categories
                    { for post.frontmatter.category.split(',').map(|cat| cat.trim()).filter(|cat| !cat.is_empty()).map(|category| {
                        let category_str = category.to_string();
                        let onclick = {
                            let category = category_str.clone();
                            Callback::from(move |_: MouseEvent| {
                                update_url_with_filter("category", &category);
                            })
                        };
                        html! {
                            <span class="category clickable" {onclick}>
                                { category }
                            </span>
                        }
                    })}

                    // Tags (excluding featured)
                    { for post.frontmatter.tags.iter().filter(|tag| *tag != "featured").map(|tag| {
                        let tag_str = tag.clone();
                        let onclick = {
                            let tag = tag_str.clone();
                            Callback::from(move |_: MouseEvent| {
                                update_url_with_filter("tag", &tag);
                            })
                        };
                        html! {
                            <span class="tag clickable" {onclick}>
                                { tag }
                            </span>
                        }
                    })}
                </div>

                // Summary if available
                if !post.frontmatter.summary.is_empty() {
                    <p class="post-view-summary">{ &post.frontmatter.summary }</p>
                }

                // Content area with floating image
                <div class="post-view-content">
                    if !post.frontmatter.image.is_empty() {
                        <img class="post-view-featured-image" src={post.frontmatter.image.clone()} alt={post.frontmatter.title.clone()} />
                    }

                    // Render the markdown content as HTML
                    <div class="post-markdown-content">
                        { Html::from_html_unchecked(AttrValue::from(render_markdown_to_html(&post.content))) }
                    </div>
                </div>

                // Add page stats display at the bottom of the post
                <PageStatsDisplay slug={AttrValue::from(post.slug.clone())} track_view={true} reading_time_seconds={calculate_reading_time(&post.content) as u32} />
            </div>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostFilters {
    pub featured: Option<bool>,
    pub category: Option<String>,
    pub tag: Option<String>,
    pub draft: Option<bool>,
}

fn get_url_filters() -> PostFilters {
    let mut featured = None;
    let mut category = None;
    let mut tag = None;
    let mut draft = None;

    if let Some(window) = window() {
        let location = window.location();
        if let Ok(search) = location.search() {
            if !search.is_empty() {
                let params: HashMap<String, String> = search[1..] // Remove the '?' prefix
                    .split('&')
                    .filter_map(|pair| {
                        let mut parts = pair.split('=');
                        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                            Some((key.to_string(), value.to_string()))
                        } else {
                            None
                        }
                    })
                    .collect();

                if let Some(featured_str) = params.get("featured") {
                    featured = Some(featured_str.to_lowercase() == "true");
                }

                if let Some(cat) = params.get("category") {
                    category = Some(cat.to_string());
                }

                if let Some(t) = params.get("tag") {
                    tag = Some(t.to_string());
                }

                if let Some(draft_str) = params.get("draft") {
                    draft = Some(draft_str.to_lowercase() == "true");
                }
            }
        }
    }

    PostFilters {
        featured,
        category,
        tag,
        draft,
    }
}

// Auto-generated function to get all post slugs from the posts directory
include!(concat!(env!("OUT_DIR"), "/post_slugs.rs"));

// Load full post content including markdown body
pub async fn load_post(slug: &str) -> Result<Post, String> {
    let url = format!("/static/posts/{}.md", slug);
    let raw_content = load_markdown_content(&url).await?;
    Ok(parse_full_post(&raw_content, slug))
}

// Parse full post including frontmatter and content
fn parse_full_post(raw_content: &str, slug: &str) -> Post {
    if raw_content.starts_with("---\n") {
        let lines: Vec<&str> = raw_content.lines().collect();
        let mut frontmatter_end = None;

        // Find the second occurrence of "---" line
        let mut found_first = false;
        for (i, line) in lines.iter().enumerate() {
            if line.trim() == "---" {
                if found_first {
                    frontmatter_end = Some(i);
                    break;
                } else {
                    found_first = true;
                }
            }
        }

        if let Some(end_line) = frontmatter_end {
            // Extract frontmatter lines (skip first --- line)
            let frontmatter_lines = &lines[1..end_line];
            let frontmatter_str = frontmatter_lines.join("\n");
            let frontmatter = parse_yaml_frontmatter(&frontmatter_str);

            // Extract content lines (skip frontmatter and closing --- line)
            let content_lines = if end_line + 1 < lines.len() {
                &lines[end_line + 1..]
            } else {
                &[]
            };
            let content = content_lines.join("\n").trim().to_string();

            return Post {
                slug: slug.to_string(),
                frontmatter,
                content,
            };
        }
    }

    // No frontmatter found, treat whole content as markdown
    Post {
        slug: slug.to_string(),
        frontmatter: PostFrontmatter::default(),
        content: raw_content.to_string(),
    }
}

// Load all posts from markdown files
async fn load_all_posts() -> Result<Vec<PostSummary>, String> {
    let mut posts = Vec::new();
    let slugs = get_all_post_slugs();

    for slug in slugs {
        match load_post_frontmatter(slug).await {
            Ok(post_summary) => posts.push(post_summary),
            Err(e) => {
                console::log_1(&format!("Failed to load post {}: {}", slug, e).into());
                // Continue loading other posts even if one fails
            }
        }
    }

    // Sort posts by date (newest first)
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

    Ok(posts)
}

// Load just the frontmatter from a post (not the full content)
async fn load_post_frontmatter(slug: &str) -> Result<PostSummary, String> {
    let url = format!("/static/posts/{}.md", slug);
    let raw_content = load_markdown_content(&url).await?;
    Ok(parse_post_frontmatter_only(&raw_content, slug))
}

// Parse just the frontmatter from markdown content
fn parse_post_frontmatter_only(raw_content: &str, slug: &str) -> PostSummary {
    if raw_content.starts_with("---\n") {
        // Look for the closing --- pattern, which might not be exactly "\n---\n"
        let lines: Vec<&str> = raw_content.lines().collect();
        let mut frontmatter_end = None;

        // Find the second occurrence of "---" line
        let mut found_first = false;
        for (i, line) in lines.iter().enumerate() {
            if line.trim() == "---" {
                if found_first {
                    frontmatter_end = Some(i);
                    break;
                } else {
                    found_first = true;
                }
            }
        }

        if let Some(end_line) = frontmatter_end {
            // Extract frontmatter lines (skip first --- line)
            let frontmatter_lines = &lines[1..end_line];
            let frontmatter_str = frontmatter_lines.join("\n");
            let frontmatter = parse_yaml_frontmatter(&frontmatter_str);

            return PostSummary {
                slug: slug.to_string(),
                frontmatter,
            };
        }
    }

    // No frontmatter found, return default
    PostSummary {
        slug: slug.to_string(),
        frontmatter: PostFrontmatter::default(),
    }
}

// Simple YAML parser for common frontmatter fields

fn parse_yaml_frontmatter(yaml_str: &str) -> PostFrontmatter {
    let mut frontmatter = PostFrontmatter::default();

    for line in yaml_str.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim().trim_matches('"').trim_matches('\'');

            match key {
                "title" => frontmatter.title = value.to_string(),
                "date" => frontmatter.date = value.to_string(),
                "summary" => frontmatter.summary = value.to_string(),
                "author" => frontmatter.author = value.to_string(),
                "category" => frontmatter.category = value.to_string(),
                "image" => frontmatter.image = value.to_string(),
                "published" => frontmatter.published = value.to_lowercase() == "true",
                "tags" => {
                    if value.starts_with('[') && value.ends_with(']') {
                        let tags_str = &value[1..value.len() - 1];
                        frontmatter.tags = tags_str
                            .split(',')
                            .map(|tag| tag.trim().trim_matches('"').trim_matches('\'').to_string())
                            .filter(|tag| !tag.is_empty())
                            .collect();
                    } else {
                        frontmatter.tags = value
                            .split(',')
                            .map(|tag| tag.trim().to_string())
                            .filter(|tag| !tag.is_empty())
                            .collect();
                    }
                }
                _ => {}
            }
        }
    }

    frontmatter
}
