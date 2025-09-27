use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <div class="header">
                <div class="header-left">
                    <a href="/"><img src="static/logo_ga.svg" alt="Logo Gertjan Assies" style="height: 50px; position: absolute; top: 10px; left: 10px;" /></a>
                    <h2>{ "gertjanassies.dev" }</h2><br/>
                    <sub>{ "ramblings of a chaotic mind" }</sub>
                </div>
                <div class="header-right">
                    <a href="/about">{ "About" }</a>
                    <a href="/contact">{ "Contact" }</a>
                </div>
            </div>
        </header>
    }
}

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

