use crate::components::aboutpage::AboutPage;
use crate::components::blogpage::BlogPage;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::homepage::HomePage;
use crate::components::notfoundpage::NotFoundPage;
use crate::components::posts::PostView;
use yew::prelude::*;
use yew_router::prelude::*;

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
        Route::About => html! { <AboutPage /> },
        Route::Post { slug } => html! {
            <PostView slug={slug} />
        },
        Route::NotFound => html! { <NotFoundPage /> },
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
