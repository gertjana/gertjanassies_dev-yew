use std::collections::HashMap;
use yew::prelude::*;

// Trait for components that can be rendered from markdown
pub trait MarkdownRenderable {
    fn render(attributes: &HashMap<String, String>) -> Html;
}
