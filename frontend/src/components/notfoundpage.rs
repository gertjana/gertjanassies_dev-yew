use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div class="not-found">
            <div class="not-found-content">
                <div class="not-found-image">
                    <img src="/static/images/404.png" alt="404 - Not Found" />
                </div>
                <div class="not-found-text">
                    <h1>{ "404 - Page Not Found" }</h1>
                    <p>{ "This is not the page you are looking for." }</p>
                </div>
            </div>
        </div>
    }
}
