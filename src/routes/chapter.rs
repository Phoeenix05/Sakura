use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct ChapterParams {
    uuid: String,
}

#[component]
pub fn Chapter(cx: Scope) -> impl IntoView {
    // Get page params
    let _ = use_params::<ChapterParams>(cx);

    view! { cx,
        <main></main>
    }
}