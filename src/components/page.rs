use pulldown_cmark::{html, Options, Parser};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Request, RequestInit, RequestMode};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageProps {
    pub content: AttrValue,
    pub children: Children,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    let markdown_content = use_state(String::new);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    let content_path = props.content.clone();

    {
        let markdown_content = markdown_content.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with(content_path.clone(), move |content_path| {
            let markdown_content = markdown_content.clone();
            let loading = loading.clone();
            let error = error.clone();
            let content_path = content_path.clone();

            spawn_local(async move {
                let content_url = format!("/static/pages/{}.md", content_path);

                match load_markdown_content(&content_url).await {
                    Ok(content) => {
                        markdown_content.set(content);
                        loading.set(false);
                    }
                    Err(err) => {
                        error.set(Some(format!("Failed to load content: {}", err)));
                        loading.set(false);
                    }
                }
            });

            || ()
        });
    }

    // Render KaTeX when page content is loaded
    {
        let loading = loading.clone();
        let markdown_content = markdown_content.clone();

        use_effect_with((*loading, (*markdown_content).clone()), move |_| {
            if !*loading && !markdown_content.is_empty() {
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
            <div class="page loading">
                <p>{"Loading..."}</p>
            </div>
        };
    }

    if let Some(error_msg) = error.as_ref() {
        return html! {
            <div class="page error">
                <p>{format!("Error: {}", error_msg)}</p>
            </div>
        };
    }

    let rendered_html = if !markdown_content.is_empty() {
        render_markdown_to_html(&markdown_content)
    } else {
        String::new()
    };

    html! {
        <div class="page">
            if !rendered_html.is_empty() {
                <div class="markdown-content">
                    {Html::from_html_unchecked(rendered_html.into())}
                </div>
            }
            <div class="page-children">
                {props.children.clone()}
            </div>
        </div>
    }
}

async fn load_markdown_content(url: &str) -> Result<String, String> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);

    let request =
        Request::new_with_str_and_init(url, &opts).map_err(|_| "Failed to create request")?;

    let window = window().ok_or("Failed to get window")?;

    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Failed to fetch")?;

    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| "Failed to cast to Response")?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let text = wasm_bindgen_futures::JsFuture::from(resp.text().map_err(|_| "Failed to get text")?)
        .await
        .map_err(|_| "Failed to read response text")?;

    text.as_string()
        .ok_or_else(|| "Response is not a string".to_string())
}

fn render_markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
