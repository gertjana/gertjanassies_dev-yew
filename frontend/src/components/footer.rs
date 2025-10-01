use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="footer">
            { "Opinions expressed here are my own and not the views of my employer or anyone else, (re)use is free, but quoting the source is appreciated." }
            <br/>
            { "This blog is licensed under a " }
            <a rel="license" href="http://creativecommons.org/licenses/by/4.0/"> { "Creative Commons Attribution 4.0 International License" }
            </a>{ ". © 2023 by Gertjan Assies" }
            </div>
        </footer>
    }
}
