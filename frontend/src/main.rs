mod app;
mod components;
mod reading_time;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
