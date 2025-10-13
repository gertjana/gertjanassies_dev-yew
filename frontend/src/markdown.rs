use crate::components::posts::Posts;
use crate::components::{Certifications, OnlinePlaces, Technologies};
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, RequestMode, Response};
use yew::prelude::*;

// Trait for components that can be rendered from markdown
pub trait MarkdownRenderable {
    fn render(attributes: &HashMap<String, String>) -> Html;
}

// Implement the trait for existing components
impl MarkdownRenderable for Technologies {
    fn render(_attributes: &HashMap<String, String>) -> Html {
        html! { <Technologies /> }
    }
}

impl MarkdownRenderable for Certifications {
    fn render(_attributes: &HashMap<String, String>) -> Html {
        html! { <Certifications /> }
    }
}

impl MarkdownRenderable for OnlinePlaces {
    fn render(_attributes: &HashMap<String, String>) -> Html {
        html! { <OnlinePlaces /> }
    }
}

impl MarkdownRenderable for Posts {
    fn render(attributes: &HashMap<String, String>) -> Html {
        let featured_only = attributes
            .get("featured_only")
            .map(|v| v == "true")
            .unwrap_or(false);

        html! { <Posts {featured_only} /> }
    }
}

// Component registry type
type ComponentRenderer = fn(&HashMap<String, String>) -> Html;

// Macro to easily register components
macro_rules! register_components {
    ($($name:literal => $component:ty),* $(,)?) => {
        {
            let mut registry = HashMap::new();
            $(
                registry.insert($name, <$component as MarkdownRenderable>::render as ComponentRenderer);
            )*
            registry
        }
    };
}

/// Create a registry of available components for markdown embedding with attribute support
///
/// To add a new component that can be embedded in markdown:
///
/// 1. Create your Yew component (e.g., `MyComponent`) with props if needed
/// 2. Add it to the components module and import it here
/// 3. Implement `MarkdownRenderable` for your component:
///    ```rust
///    impl MarkdownRenderable for MyComponent {
///        fn render(attributes: &HashMap<String, String>) -> Html {
///            // Parse attributes as needed
///            let my_prop = attributes.get("my_prop").unwrap_or("default");
///            html! { <MyComponent {my_prop} /> }
///        }
///    }
///    ```
/// 4. Add it to the registry below using the pattern:
///    `"ComponentName" => ComponentType,`
/// 5. Use it in markdown files with: `<ComponentName />` or `<ComponentName my_prop="value" />`
///
/// The system supports:
/// - Components with or without attributes
/// - Attribute parsing with key="value" syntax
/// - Error handling for unknown components
/// - Helpful error messages showing available components
pub fn get_component_registry() -> HashMap<&'static str, ComponentRenderer> {
    register_components! {
        "Technologies" => Technologies,
        "Certifications" => Certifications,
        "OnlinePlaces" => OnlinePlaces,
        "Posts" => Posts,
        // Add new components here following the same pattern:
        // "MyNewComponent" => MyNewComponent,
    }
}

pub async fn load_markdown_content(url: &str) -> Result<String, String> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;

    let window = window().ok_or("No global window exists")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch failed: {:?}", e))?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|e| format!("Failed to cast to Response: {:?}", e))?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let text_promise = resp
        .text()
        .map_err(|e| format!("Failed to get text promise: {:?}", e))?;

    let text_value = JsFuture::from(text_promise)
        .await
        .map_err(|e| format!("Failed to get text: {:?}", e))?;

    text_value
        .as_string()
        .ok_or_else(|| "Response text is not a string".to_string())
}

fn get_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options
}

pub fn render_markdown_to_html(markdown: &str) -> String {
    let options = get_options();

    let parser = Parser::new_ext(markdown, options);
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
                events.push(Event::Html(
                    format!(
                        r#"<pre class="language-{}"><code class="language-{}">"#,
                        code_language, code_language
                    )
                    .into(),
                ));
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

// Structure to hold markdown content parts and component positions
#[derive(Clone)]
pub struct MarkdownPart {
    pub content: String,
    pub is_component: bool,
    pub component_name: Option<String>,
    pub attributes: HashMap<String, String>,
}

// Parse attributes from a component tag string
fn parse_component_attributes(tag_content: &str) -> (String, HashMap<String, String>) {
    let mut attributes = HashMap::new();
    let parts: Vec<&str> = tag_content.split_whitespace().collect();

    if parts.is_empty() {
        return (String::new(), attributes);
    }

    let component_name = parts[0].to_string();

    // Parse attributes in the form key="value" or key='value'
    for part in parts.iter().skip(1) {
        if let Some(eq_pos) = part.find('=') {
            let key = part[..eq_pos].to_string();
            let value_part = &part[eq_pos + 1..];

            // Remove quotes if present
            let value = if (value_part.starts_with('"') && value_part.ends_with('"'))
                || (value_part.starts_with('\'') && value_part.ends_with('\''))
            {
                value_part[1..value_part.len() - 1].to_string()
            } else {
                value_part.to_string()
            };

            attributes.insert(key, value);
        }
    }

    (component_name, attributes)
}

// Parse markdown with component tags and return structured parts
pub fn parse_markdown_with_components(markdown: &str) -> Vec<MarkdownPart> {
    let mut parts = Vec::new();
    let lines: Vec<&str> = markdown.lines().collect();
    let mut current_content = String::new();

    for line in lines {
        let trimmed = line.trim();

        // Check if line contains a component tag
        if trimmed.starts_with('<') && trimmed.ends_with(" />") {
            // If we have accumulated content, add it as a markdown part
            if !current_content.is_empty() {
                parts.push(MarkdownPart {
                    content: current_content.trim().to_string(),
                    is_component: false,
                    component_name: None,
                    attributes: HashMap::new(),
                });
                current_content.clear();
            }

            // Extract component name and attributes
            let tag_content = trimmed.trim_start_matches('<').trim_end_matches(" />");

            let (component_name, attributes) = parse_component_attributes(tag_content);

            // Add component part
            parts.push(MarkdownPart {
                content: String::new(),
                is_component: true,
                component_name: Some(component_name),
                attributes,
            });
        } else {
            // Accumulate regular markdown content
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // Add any remaining content
    if !current_content.is_empty() {
        parts.push(MarkdownPart {
            content: current_content.trim().to_string(),
            is_component: false,
            component_name: None,
            attributes: HashMap::new(),
        });
    }

    parts
}

// Render a component by name using the dynamic registry
pub fn render_component_by_name(name: &str, attributes: &HashMap<String, String>) -> Html {
    let registry = get_component_registry();

    match registry.get(name) {
        Some(renderer) => renderer(attributes),
        None => {
            // Log available components for debugging
            let available: Vec<&str> = registry.keys().copied().collect();
            html! {
                <div class="unknown-component">
                    <p>{format!("Unknown component: '{}'", name)}</p>
                    <p><small>{format!("Available components: {}", available.join(", "))}</small></p>
                </div>
            }
        }
    }
}
