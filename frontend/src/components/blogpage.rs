use super::Page;
use crate::components::posts::Posts;
use yew::prelude::*;

#[function_component(BlogPage)]
pub fn blog_page() -> Html {
    html! {
        <div class="blog-page">
            <Page content="blog">
                <Posts featured_only={false} />
            </Page>
        </div>
    }
}
