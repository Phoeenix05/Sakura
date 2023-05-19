use anyhow::Result;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen;
use wasm_bindgen::prelude::*;

use crate::util::MangaFeed;

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
        move || {
            let uuid = params.get().unwrap().uuid.to_owned();
            uuid
        },
        move |uuid| async move {
            let data = invoke("fetch_manga_data", to_value(&MangaParams { uuid }).unwrap())
                .await
                .as_string()
                .unwrap();
            let json = serde_json::from_str::<MangaFeed>(&data).unwrap();
            json
        },
    );

    create_effect(cx, move |_| {
        log!("params = {:#?}", params.get().unwrap());
    });

    let data_display = move || match data.read(cx) {
        Some(data) => view! { cx, <><pre>{ data.to_string() }</pre></>},
        None => view! { cx, <>"Loading..."</> },
    };

    view! { cx,
        <div id="manga_data">
            <Transition fallback=move || view! { cx, <>"Loading..."</>}>
                { data_display }
            </Transition>
        </div>
    }
}
