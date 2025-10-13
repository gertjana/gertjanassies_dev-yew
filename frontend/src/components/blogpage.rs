use super::Page;
use crate::hooks::use_document_title;
use yew::prelude::*;

#[function_component(BlogPage)]
pub fn blog_page() -> Html {
    use_document_title("Blog - gertjanassies.dev");

    html! {
        <div class="blog-page">
            <Page content="blog" />
        </div>
    }
}
