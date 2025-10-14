use crate::components::posts::Posts;
use crate::components::{Certifications, OnlinePlaces, Technologies};
use crate::traits::MarkdownRenderable;
use once_cell::sync::Lazy;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use regex::Regex;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, RequestMode, Response};
use yew::prelude::*;

// Lazy-compiled regex patterns for better performance
// These are compiled once when first accessed and reused across all calls

/// Regex to match self-closing component tags
/// Pattern explanation:
/// - `<([A-Z][a-zA-Z0-9]*)` - Component name starting with uppercase (capture group 1)
/// - `((?:\s+[a-zA-Z_][a-zA-Z0-9_]*\s*=\s*(?:'[^']*'|"[^"]*"))*)?` - Optional attributes (capture group 2)
/// - `\s*/>` - Optional whitespace and closing />
static COMPONENT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"<([A-Z][a-zA-Z0-9]*)((?:\s+[a-zA-Z_][a-zA-Z0-9_]*\s*=\s*(?:'[^']*'|"[^"]*"))*)\s*/>"#,
    )
    .expect("Component regex pattern should be valid")
});

/// Regex to match individual attributes within component tags
/// Matches: attr="value" or attr='value'
/// - `([a-zA-Z_][a-zA-Z0-9_]*)` - Attribute name (capture group 1)
/// - `\s*=\s*` - Equals sign with optional whitespace
/// - `(?:"([^"]*)"|'([^']*)')` - Quoted value, either double (group 2) or single (group 3) quotes
static ATTRIBUTE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(?:"([^"]*)"|'([^']*)')"#)
        .expect("Attribute regex pattern should be valid")
});

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

// Parse attributes from a component tag string using regex
fn parse_component_attributes(tag_content: &str) -> (String, HashMap<String, String>) {
    let mut attributes = HashMap::new();
    let parts: Vec<&str> = tag_content.split_whitespace().collect();

    if parts.is_empty() {
        return (String::new(), attributes);
    }

    let component_name = parts[0].to_string();

    // Use lazy static regex to parse attributes more robustly
    // Join the remaining parts back into a string for regex parsing
    let attr_string = if parts.len() > 1 {
        parts[1..].join(" ")
    } else {
        String::new()
    };

    for cap in ATTRIBUTE_REGEX.captures_iter(&attr_string) {
        let key = cap.get(1).unwrap().as_str().to_string();
        // Check which quote group matched (group 2 for double quotes, group 3 for single quotes)
        let value = if let Some(double_quoted) = cap.get(2) {
            double_quoted.as_str().to_string()
        } else if let Some(single_quoted) = cap.get(3) {
            single_quoted.as_str().to_string()
        } else {
            String::new()
        };

        attributes.insert(key, value);
    }

    (component_name, attributes)
}

/// Parse markdown with component tags using regex and return structured parts
///
/// This function uses regex to detect component tags anywhere in the markdown content,
/// not just on separate lines. It supports:
/// - Simple components: `<ComponentName />`
/// - Components with attributes: `<ComponentName attr="value" />`
/// - Inline components within paragraphs
/// - Multiple components on the same line
///
/// The regex pattern matches:
/// - Component names starting with uppercase letter: `[A-Z][a-zA-Z0-9]*`
/// - Optional attributes with quoted values: `attr="value"` or `attr='value'`
/// - Self-closing syntax: `/>`
pub fn parse_markdown_with_components(markdown: &str) -> Vec<MarkdownPart> {
    let mut parts = Vec::new();
    let mut last_end = 0;

    // Use the lazy-compiled regex for better performance
    // Find all component matches
    for component_match in COMPONENT_REGEX.find_iter(markdown) {
        let start = component_match.start();
        let end = component_match.end();

        // Add any markdown content before this component
        if start > last_end {
            let content = &markdown[last_end..start];
            if !content.trim().is_empty() {
                parts.push(MarkdownPart {
                    content: content.to_string(),
                    is_component: false,
                    component_name: None,
                    attributes: HashMap::new(),
                });
            }
        }

        // Parse the component
        let full_tag = component_match.as_str();
        let tag_content = &full_tag[1..full_tag.len() - 2]; // Remove < and />
        let (component_name, attributes) = parse_component_attributes(tag_content);

        parts.push(MarkdownPart {
            content: String::new(),
            is_component: true,
            component_name: Some(component_name),
            attributes,
        });

        last_end = end;
    }

    // Add any remaining markdown content after the last component
    if last_end < markdown.len() {
        let content = &markdown[last_end..];
        if !content.trim().is_empty() {
            parts.push(MarkdownPart {
                content: content.to_string(),
                is_component: false,
                component_name: None,
                attributes: HashMap::new(),
            });
        }
    }

    // If no components were found, add the entire markdown as content
    if parts.is_empty() && !markdown.trim().is_empty() {
        parts.push(MarkdownPart {
            content: markdown.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_component_detection() {
        let markdown = "# Hello\n\n<Technologies />\n\nMore content";
        let parts = parse_markdown_with_components(markdown);

        assert_eq!(parts.len(), 3);
        assert!(!parts[0].is_component);
        assert_eq!(parts[0].content.trim(), "# Hello");

        assert!(parts[1].is_component);
        assert_eq!(parts[1].component_name.as_ref().unwrap(), "Technologies");
        assert!(parts[1].attributes.is_empty());

        assert!(!parts[2].is_component);
        assert_eq!(parts[2].content.trim(), "More content");
    }

    #[test]
    fn test_component_with_attributes() {
        let markdown = r#"Before <Technologies type="tools" /> After"#;
        let parts = parse_markdown_with_components(markdown);

        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].content.trim(), "Before");

        assert!(parts[1].is_component);
        assert_eq!(parts[1].component_name.as_ref().unwrap(), "Technologies");
        assert_eq!(parts[1].attributes.get("type").unwrap(), "tools");

        assert_eq!(parts[2].content.trim(), "After");
    }

    #[test]
    fn test_multiple_components_same_line() {
        let markdown = r#"<Certifications /> and <OnlinePlaces />"#;
        let parts = parse_markdown_with_components(markdown);

        assert_eq!(parts.len(), 3);
        assert!(parts[0].is_component);
        assert_eq!(parts[0].component_name.as_ref().unwrap(), "Certifications");

        assert_eq!(parts[1].content.trim(), "and");

        assert!(parts[2].is_component);
        assert_eq!(parts[2].component_name.as_ref().unwrap(), "OnlinePlaces");
    }

    #[test]
    fn test_component_attribute_parsing() {
        let (name, attrs) = parse_component_attributes(r#"Technologies type="languages""#);
        assert_eq!(name, "Technologies");
        assert_eq!(attrs.get("type").unwrap(), "languages");

        let (name, attrs) = parse_component_attributes("Posts featured_only='true'");
        assert_eq!(name, "Posts");
        assert_eq!(attrs.get("featured_only").unwrap(), "true");

        let (name, attrs) =
            parse_component_attributes(r#"MyComponent attr1="value1" attr2='value2'"#);
        assert_eq!(name, "MyComponent");
        assert_eq!(attrs.get("attr1").unwrap(), "value1");
        assert_eq!(attrs.get("attr2").unwrap(), "value2");
    }
}
