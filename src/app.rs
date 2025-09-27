use yew::prelude::*;
use crate::components::general::{Header, Footer};
use crate::components::posts::Posts;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header />
            <main>
                <Posts />
            </main>
        <Footer />
        </>
    }
}
