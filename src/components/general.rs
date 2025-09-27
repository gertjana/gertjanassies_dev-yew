use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <div class="header">
                <div class="header-left">
                    <Link<Route> to={Route::Home}>
                        <img src="/static/logo_ga.svg" alt="Logo Gertjan Assies" style="height: 50px; position: absolute; top: 10px; left: 10px;" />
                    </Link<Route>>
                    <h2>{ "gertjanassies.dev" }</h2><br/>
                    <sub>{ "ramblings of a chaotic mind" }</sub>
                </div>
                <div class="header-right">
                <nav>
                    <Link<Route> to={Route::Home}>{ "home" }</Link<Route>>
                    <Link<Route> to={Route::Blog}>{ "blog" }</Link<Route>>
                    <Link<Route> to={Route::About}>{ "about" }</Link<Route>>
                    <span>{ "categories: " }</span>
                    <a href="/blog?category=code">{ "code" }</a>
                    <a href="/blog?category=make">{ "make" }</a>
                    <a href="/blog?category=tooling">{ "tooling" }</a>
                    <a href="/blog?category=life">{ "life" }</a>
                </nav>
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

