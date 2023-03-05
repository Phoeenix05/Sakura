// mod app;
pub mod components;
pub mod routes;

use components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
