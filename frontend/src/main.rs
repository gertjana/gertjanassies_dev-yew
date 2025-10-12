mod app;
mod components;
mod hooks;
mod reading_time;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
