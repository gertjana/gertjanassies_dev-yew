use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::markdown::{load_markdown_content, render_markdown_to_html};
use super::page_stats_display::PageStatsDisplay;
use crate::reading_time::calculate_reading_time;

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
            // Add page stats display at the bottom
            <PageStatsDisplay slug={props.content.clone()} track_view={true} reading_time_seconds={calculate_reading_time(&markdown_content) as u32} />
        </div>
    }
}
