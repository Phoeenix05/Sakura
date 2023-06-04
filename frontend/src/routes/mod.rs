use leptos::*;
use leptos_router::{Route, Router, Routes, A};

mod chapter;
mod manga;

use chapter::*;
use manga::*;

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <A href="manga/e78a489b-6632-4d61-b00b-5206f5b8b22b">"Hello, World"</A>
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
