// mod app;
pub mod components;
pub mod routes;
pub mod app;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
