use crate::components::aboutpage::AboutPage;
use crate::components::auth_wrapper::AuthWrapper;
use crate::components::blogpage::BlogPage;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::homepage::HomePage;
use crate::components::notfoundpage::NotFoundPage;
use crate::components::page::Page;
use crate::components::posts::PostView;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/about")]
    About,
    #[at("/post/:slug")]
    Post { slug: String },
    #[at("/private/*")]
    Private,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Private {
    #[at("/private/resume")]
    Resume,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Blog => html! { <BlogPage /> },
        Route::About => html! { <AboutPage /> },
        Route::Post { slug } => html! {
            <PostView slug={slug} />
        },
        Route::Private => html! { <Switch<Private> render={switch_private} /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

fn switch_private(routes: Private) -> Html {
    match routes {
        Private::Resume => html! {
            <AuthWrapper>
                <Page content={AttrValue::from("resume")} />
            </AuthWrapper>
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
