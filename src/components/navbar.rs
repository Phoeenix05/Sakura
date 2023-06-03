use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="navbar">
            <A exact=true href="/">{"Home"}</A>
            <A href="/manga/c288b108-5162-4065-aa3a-5857ec38c8d9">{"Manga"}</A>
            <A href="/chapter/71e9509e-d67e-49e5-b552-e03e67d04ae7">{"Chapter"}</A>
        </div>
    }
}
