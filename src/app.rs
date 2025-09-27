use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::general::{Header, Footer};
use crate::components::posts::{PostView, HomePage, BlogPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/about")]
    About,
    #[at("/post/:slug")]
    Post { slug: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Blog => html! { <BlogPage /> },
        Route::About => html! { 
            <div class="about-page">
                <h1>{ "About" }</h1>
                <p>{ "This is the about page. Coming soon!" }</p>
            </div>
        },
        Route::Post { slug } => html! { 
            <PostView slug={slug} />
        },
        Route::NotFound => html! { 
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
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <main>
                <Switch<Route> render={switch} />
            </main>
            <Footer />
        </BrowserRouter>
    }
}
