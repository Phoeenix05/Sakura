use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="navbar">
            <A exact=true href="/">{"Home"}</A>
            <A href="/manga/45b8d08e-b3ca-4aa7-bb14-120a1ed3c72f">{"Manga"}</A>
        </div>
    }
}
