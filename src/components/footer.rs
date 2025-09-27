use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
        <div class="footer">
            <p>{ "© 2024 My Blog too" }</p>
        </div>
        </footer>
    }
}
