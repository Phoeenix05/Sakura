use leptos::*;
use leptos_router::{Router, Routes, Route, A};

mod manga;
mod chapter;

use manga::*;
use chapter::*;

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <A href="manga/c288b108-5162-4065-aa3a-5857ec38c8d9">"Hello, World"</A>
    }
}

#[component]
pub fn Pages(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Routes>
                <Route path="" view=move |cx| view! { cx, <Home /> } />
                <Route path="manga/:id" view=move |cx| view! { cx, <Manga /> } />
                <Route path="chapter/:id" view=move |cx| view! { cx, <Chapter /> } />
            </Routes>
        </Router>
    }
}
