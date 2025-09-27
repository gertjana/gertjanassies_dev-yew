use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <div class="about-page">
            <h1>{ "About" }</h1>
            <p>{ "This is the about page. Coming soon!" }</p>
        </div>
    }
}
