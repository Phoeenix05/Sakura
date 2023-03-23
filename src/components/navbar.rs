use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    // cfg_if::cfg_if! {
    //     if #[cfg(target_os = "macos")] {
    //         let os = "darwin";
    //     } else {
    //         let os = "not_darwin";
    //     }
    // };

    view! { cx,
        <div id={format!("navbar ")}>
            <A exact=true href="/">{"Home"}</A>
            <A href="/manga/45b8d08e-b3ca-4aa7-bb14-120a1ed3c72f">{"Manga"}</A>
        </div>
    }
}
