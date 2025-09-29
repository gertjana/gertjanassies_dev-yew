use super::{OnlinePlaces, Page};
use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <div class="about-page">
            <Page content="about">
                <OnlinePlaces />
            </Page>
        </div>
    }
}
