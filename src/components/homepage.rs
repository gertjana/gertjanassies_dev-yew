use super::Page;
use crate::components::posts::Posts;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="home-page">
            <Page content="home">
                <Posts featured_only={true} />
            </Page>
        </div>
    }
}
