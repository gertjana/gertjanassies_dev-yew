use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="footer">
                <p>{ "© 2025 Addictive Software. All rights reserved. this code is licensed under the MIT License." }</p>
            </div>
        </footer>
    }
}
