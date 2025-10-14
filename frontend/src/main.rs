mod app;
mod components;
mod hooks;
mod markdown;
mod reading_time;
mod traits;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
