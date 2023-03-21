use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div data-tauri-drag-region id="navbar">
            <A href="/manga/45b8d08e-b3ca-4aa7-bb14-120a1ed3c72f">{"Manga"}</A>
        </div>
    }
}
