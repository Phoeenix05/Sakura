use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct SettingsParams {
    setting: String,
}

#[component]
pub fn Settings(cx: Scope) -> impl IntoView {
    // Get page params
    let _ = use_params::<SettingsParams>(cx);

    view! { cx,
        <main></main>
    }
}
