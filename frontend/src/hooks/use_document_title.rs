use web_sys::window;
use yew::prelude::*;

#[hook]
pub fn use_document_title(title: &str) {
    let title = title.to_string();

    use_effect_with(title.clone(), move |title| {
        if let Some(document) = window().and_then(|w| w.document()) {
            document.set_title(title);
        }
        // || {}
    });
}
