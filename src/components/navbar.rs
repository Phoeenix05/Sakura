use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div data-tauri-drag-region id="navbar">
        </div>
    }
}
