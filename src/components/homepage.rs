use crate::components::posts::Posts;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="home-page">
            <div class="home-header">
              <p>{ "This is my personal space where I talk about technology, coding, the maker space and anything else that interests me." }</p>
            </div>
            <Posts featured_only={true} />
        </div>
    }
}
