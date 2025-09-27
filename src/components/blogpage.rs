use crate::components::posts::Posts;
use yew::prelude::*;

#[function_component(BlogPage)]
pub fn blog_page() -> Html {
    html! {
        <div class="blog-page">
            <div class="blog-header">
                <p>{ "Explore all articles, tutorials, and insights. Use the filters below to find posts by category, tags, or search for specific topics that interest you." }</p>
            </div>
            <Posts featured_only={false} />
        </div>
    }
}
