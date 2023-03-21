use anyhow::Result;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Params, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct MangaParams {
    uuid: String,
}

#[component]
pub fn Manga(cx: Scope) -> impl IntoView {
    let params = use_params::<MangaParams>(cx);
    let data = create_resource(
        cx,
        move || params.get().map(|p| p.uuid).ok(),
        move |uuid| async move {
            let args = to_value(&MangaParams {
                uuid: uuid.unwrap(),
            });
            let data = invoke("fetch_manga_data", args.unwrap()).await;
            data.as_string().unwrap()
        },
    );

    create_effect(cx, move |_| {
        log!("params = {:#?}", params.get());
    });

    let data_display = move || match data.read(cx) {
        Some(data) => view! { cx, <pre>{ data }</pre>},
        None => view! { cx, <pre>"Loading..."</pre> },
    };

    view! { cx,
        <div>
            <Transition fallback=move || view! { cx, <p>"Loading..."</p> }>
                { data_display }
            </Transition>
        </div>
    }
}
