use leptos::*;
use leptos_router::*;

mod app;
mod components;
mod routes;

use app::*;
use components::*;

#[component]
fn Main(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Routes>
                <Route path="/" view=move |cx| view! { cx, <App /> } />
                <Route path="/manga/:id" view=move |_| view! { cx, <></> } />
                <Route path="/chapter/:id" view=move |_| view! { cx, <></> } />
                <Route path="/settings/:setting" view=move |_| view! { cx, <></> } />
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <>
                <Navbar />
                <Main />
            </> 
        }
    })
}
