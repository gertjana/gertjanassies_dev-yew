use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, RequestMode, Response};

/// Load markdown content from a URL
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

/// Render markdown to HTML with basic options (no syntax highlighting)
pub fn render_markdown_to_html(markdown: &str) -> String {
    let options = get_options();
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Render markdown to HTML with syntax highlighting support for code blocks
pub fn markdown_to_html_with_highlighting(markdown: &str) -> String {
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
