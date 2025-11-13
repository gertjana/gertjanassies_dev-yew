use web_sys::window;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MetaData {
    pub title: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub image: Option<String>,
    pub article_author: Option<String>,
    pub article_published_time: Option<String>,
    pub article_tag: Option<Vec<String>>,
    pub twitter_card_type: Option<String>,
}

impl Default for MetaData {
    fn default() -> Self {
        Self {
            title: "gertjanassies.dev".to_string(),
            description: Some(
                "Gertjan Assies personal blog, articles about coding and the maker space"
                    .to_string(),
            ),
            url: Some("https://gertjanassies.dev".to_string()),
            image: Some("https://gertjanassies.dev/static/logo_ga.svg".to_string()),
            article_author: None,
            article_published_time: None,
            article_tag: None,
            twitter_card_type: Some("summary".to_string()),
        }
    }
}

#[hook]
pub fn use_meta_tags(meta_data: MetaData) {
    use_effect_with(meta_data.clone(), move |meta_data| {
        if let Some(document) = window().and_then(|w| w.document()) {
            // Update document title
            document.set_title(&meta_data.title);

            // Helper function to update or create meta tag
            let update_meta_tag = |name: &str, property: Option<&str>, content: &str| {
                let selector = if let Some(prop) = property {
                    format!(r#"meta[property="{}"]"#, prop)
                } else {
                    format!(r#"meta[name="{}"]"#, name)
                };

                if let Ok(Some(existing)) = document.query_selector(&selector) {
                    existing.set_attribute("content", content).ok();
                } else {
                    // Create new meta tag
                    if let Ok(meta) = document.create_element("meta") {
                        if let Some(prop) = property {
                            meta.set_attribute("property", prop).ok();
                        } else {
                            meta.set_attribute("name", name).ok();
                        }
                        meta.set_attribute("content", content).ok();

                        if let Some(head) = document.head() {
                            head.append_child(&meta).ok();
                        }
                    }
                }
            };

            // Update basic meta tags
            let description = meta_data.description.as_deref().unwrap_or("");
            update_meta_tag("description", None, description);

            // Update OpenGraph meta tags
            update_meta_tag("", Some("og:title"), &meta_data.title);
            if let Some(ref description) = meta_data.description {
                update_meta_tag("", Some("og:description"), description);
            }
            if let Some(ref url) = meta_data.url {
                update_meta_tag("", Some("og:url"), url);
            }
            if let Some(ref image) = meta_data.image {
                update_meta_tag("", Some("og:image"), image);
            }

            // Set og:type based on whether it's an article
            let og_type = if meta_data.article_published_time.is_some() {
                "article"
            } else {
                "website"
            };
            update_meta_tag("", Some("og:type"), og_type);

            // Update article-specific meta tags
            if let Some(ref author) = meta_data.article_author {
                update_meta_tag("", Some("article:author"), author);
            }
            if let Some(ref published_time) = meta_data.article_published_time {
                update_meta_tag("", Some("article:published_time"), published_time);
            }
            if let Some(ref tags) = meta_data.article_tag {
                // Remove existing article:tag meta tags
                let existing_tags = document.query_selector_all(r#"meta[property="article:tag"]"#);
                if let Ok(node_list) = existing_tags {
                    for i in 0..node_list.length() {
                        if let Some(node) = node_list.get(i) {
                            if let Some(parent) = node.parent_node() {
                                parent.remove_child(&node).ok();
                            }
                        }
                    }
                }

                // Add new article:tag meta tags
                for tag in tags {
                    update_meta_tag("", Some("article:tag"), tag);
                }
            }

            // Update Twitter meta tags
            update_meta_tag("twitter:title", None, &meta_data.title);
            if let Some(ref description) = meta_data.description {
                update_meta_tag("twitter:description", None, description);
            }
            if let Some(ref card_type) = meta_data.twitter_card_type {
                update_meta_tag("twitter:card", None, card_type);
            }
            if let Some(ref image) = meta_data.image {
                update_meta_tag("twitter:image", None, image);
            }
        }

        || { /* cleanup: restore default meta tags when component unmounts */ }
    });
}
