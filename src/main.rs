mod app;
mod components;
mod hooks;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
