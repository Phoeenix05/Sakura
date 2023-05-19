use leptos::*;
use leptos_router::*;

mod components;
mod routes;
mod util;
mod json;

use components::*;
use routes::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <Router>
                <div data-tauri-drag-region id="drag_region"></div>
                <Navbar />
                <Routes>
                    <Route path="/" view=move |cx| view! { cx, <Index /> } />
                    <Route path="/manga/:uuid" view=move |cx| view! { cx, <Manga /> } />
                    <Route path="/chapter/:uuid" view=move |cx| view! { cx, <Chapter /> } />
                    <Route path="/settings/:setting" view=move |cx| view! { cx, <Settings /> } />
                </Routes>
            </Router>
        }
    })
}
