use super::Page;
use crate::hooks::{use_meta_tags, MetaData};
use yew::prelude::*;

#[function_component(BlogPage)]
pub fn blog_page() -> Html {
    let meta_data = MetaData {
        title: "Blog - gertjanassies.dev".to_string(),
        url: Some("https://gertjanassies.dev/blog".to_string()),
        ..Default::default()
    };
    use_meta_tags(meta_data);

    html! {
        <div class="blog-page">
            <Page content="blog" />
        </div>
    }
}
