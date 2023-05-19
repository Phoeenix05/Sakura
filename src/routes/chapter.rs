use leptos::*;
use leptos_router::*;



#[derive(Params, PartialEq, Clone, Debug)]
pub struct ChapterParams {
    uuid: String,
}

#[component]
pub fn Chapter(cx: Scope) -> impl IntoView {
    let _params = use_params::<ChapterParams>(cx);

    view! { cx,
        <main></main>
    }
}
