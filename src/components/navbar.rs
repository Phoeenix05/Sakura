use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="navbar">
            <A exact=true href="/">{"Home"}</A>
            <A href="/manga/e6eb6bd0-0285-4fac-a6da-9bc4234ac1bb">{"Manga"}</A>
        </div>
    }
}
