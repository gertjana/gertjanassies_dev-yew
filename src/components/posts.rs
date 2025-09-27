use yew::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::{window, Request, RequestInit, RequestMode, Response, console};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use std::collections::HashMap;
use pulldown_cmark::{Parser, html, Event, Tag, TagEnd, CodeBlockKind};

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

// #[function_component(Post)]
// pub fn post(props: &Props) -> Html {
//     html! {
//         <article>
//             <h1>{ format!("Post: {}", props.slug) }</h1>
//             <p>{ "This is a placeholder for the post content." }</p>
//         </article>
//     }
// }

// Helper function to update URL with new query parameters, removing all other filters
fn update_url_with_filter(key: &str, value: &str) {
    if let Some(window) = window() {
        // Always navigate to /blog when filtering, regardless of current page
        let new_url = format!("/blog?{}={}", key, value);
        
        if let Ok(history) = window.history() {
            let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));
            // Trigger a page reload to update the component
            let _ = window.location().reload();
        }
    }
}

#[function_component(Posts)]
pub fn posts(props: &PostsProps) -> Html {
    let posts_data = use_state(|| Vec::<PostSummary>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Load all posts on component mount
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

    // Get filters from URL query parameters
    let filters = get_url_filters();

    // Check if any filters are applied (excluding draft filter)
    let has_filters = filters.featured.is_some() || filters.category.is_some() || filters.tag.is_some();
    
    // Filter posts based on query parameters and component props
    let mut filtered_posts: Vec<&PostSummary> = posts_data
        .iter()
        .filter(|post| {
            // Handle draft posts - show drafts only if draft=true in query string
            let show_drafts = filters.draft.unwrap_or(false);
            if !show_drafts && !post.frontmatter.published {
                return false; // Hide drafts by default
            }
            // When draft=true, show both published and draft posts (no filtering here)
            
            // If featured_only prop is true, show only posts with "featured" tag
            if props.featured_only {
                return post.frontmatter.tags.contains(&"featured".to_string());
            }
            
            // If no URL filters are applied and not featured_only, show all published posts
            if !has_filters {
                return true;
            }
            
            // Check featured filter - check if post has "featured" tag
            if let Some(required_featured) = filters.featured {
                let has_featured_tag = post.frontmatter.tags.contains(&"featured".to_string());
                if has_featured_tag != required_featured {
                    return false;
                }
            }
            
            // Check category filter - check if post contains the required category
            if let Some(ref required_category) = filters.category {
                let post_categories: Vec<&str> = post.frontmatter.category
                    .split(',')
                    .map(|cat| cat.trim())
                    .collect();
                if !post_categories.contains(&required_category.as_str()) {
                    return false;
                }
            }
            
            // Check tag filter
            if let Some(ref required_tag) = filters.tag {
                if !post.frontmatter.tags.contains(required_tag) {
                    return false;
                }
            }
            
            true
        })
        .collect();
    
    // Sort by date descending (newest first)
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
                    <a href={format!("/post/{}", post.slug)}>{ &post.frontmatter.title }</a>
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
                        <a href={format!("/post/{}", post.slug)}>{ &post.frontmatter.title }</a>
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
                        { Html::from_html_unchecked(AttrValue::from(markdown_to_html(&post.content))) }
                    </div>
                </div>
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
    
    PostFilters { featured, category, tag, draft }
}

// Helper function to get all post slugs (you'll need to maintain this list)
fn get_all_post_slugs() -> Vec<&'static str> {
    vec![
        "210609_practical_recursion_in_elixir",
        "210610_TOTP_Exercise",
        "210619_online_meetings",
        "210813_smaller_docker_containers",
        "211029_too_many_authenticating_failures",
        "211105_why_i_sold_my_vanmoof",
        "220528_optional_go",
        "220801_online_meetings2",
        "220918_online_meetings3",
        "221102_eks_fine_grained_access",
        "230201_an_opionated_terminal",
        "230627_new_blog",
        "230714_implement_pageviews",
        "231010_redis_migration",
        "240101_rust_on_esp32",
        "240125_rust_on_esp32_2_hardware",
        "240226_rust_on_esp32_3_mqtt",
        "240830_simple_ai_cmdline",
        "250114_inky_impressions",
        "250429_vibe_coding",
    ]
}

// Load full post content including markdown body
pub async fn load_post(slug: &str) -> Result<Post, String> {
    let url = format!("/static/posts/{}.md", slug);
    
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);
    
    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;
    
    let window = web_sys::window().ok_or("No global window exists")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch failed: {:?}", e))?;
    
    let resp: Response = resp_value.dyn_into()
        .map_err(|e| format!("Failed to cast to Response: {:?}", e))?;
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    
    let text_promise = resp.text()
        .map_err(|e| format!("Failed to get text promise: {:?}", e))?;
    
    let text_value = JsFuture::from(text_promise)
        .await
        .map_err(|e| format!("Failed to get text: {:?}", e))?;
    
    let raw_content = text_value.as_string()
        .ok_or_else(|| "Response text is not a string".to_string())?;
    
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
    
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);
    
    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;
    
    let window = web_sys::window().ok_or("No global window exists")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch failed: {:?}", e))?;
    
    let resp: Response = resp_value.dyn_into()
        .map_err(|e| format!("Failed to cast to Response: {:?}", e))?;
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    
    let text_promise = resp.text()
        .map_err(|e| format!("Failed to get text promise: {:?}", e))?;
    
    let text_value = JsFuture::from(text_promise)
        .await
        .map_err(|e| format!("Failed to get text: {:?}", e))?;
    
    let raw_content = text_value.as_string()
        .ok_or_else(|| "Response text is not a string".to_string())?;
    
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
// Convert markdown to HTML using pulldown-cmark with syntax highlighting support
fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    
    // Process events and add Prism classes to code blocks
    let mut events = Vec::new();
    
    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                let code_language = if lang.is_empty() {
                    "text".to_string()
                } else {
                    lang.to_string()
                };
                events.push(Event::Html(format!(r#"<pre class="language-{}"><code class="language-{}">"#, code_language, code_language).into()));
            }
            Event::End(TagEnd::CodeBlock) => {
                events.push(Event::Html("</code></pre>".into()));
            }
            _ => {
                events.push(event);
            }
        }
    }
    
    html::push_html(&mut html_output, events.into_iter());
    html_output
}

fn parse_yaml_frontmatter(yaml_str: &str) -> PostFrontmatter {
    let mut frontmatter = PostFrontmatter::default();
    
    for line in yaml_str.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        
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
                        let tags_str = &value[1..value.len()-1];
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
                },
                _ => {}
            }
        }
    }
    
    frontmatter
}

// Home page component
#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="home-page">
            <div class="home-header">
              <h1>{ "Home page" }</h1>
              <p>{ "This is my personal space where I talk about technology, coding, the maker space and anything else that interests me." }</p>            
            </div>
            <Posts featured_only={true} />
        </div>
    }
}

// Blog page component  
#[function_component(BlogPage)]
pub fn blog_page() -> Html {
    html! {
        <div class="blog-page">
            <div class="blog-header">
                <h1>{ "All Posts" }</h1>
                <p>{ "Explore all articles, tutorials, and insights. Use the filters below to find posts by category, tags, or search for specific topics that interest you." }</p>
            </div>
            <Posts featured_only={false} />
        </div>
    }
}
