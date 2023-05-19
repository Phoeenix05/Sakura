use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct SettingsParams {
    setting: String,
}

#[component]
pub fn Settings(cx: Scope) -> impl IntoView {
    let params = use_params::<SettingsParams>(cx);
    let _page_name = params.get().unwrap().setting;

    view! { cx,
        <main></main>
    }
}
