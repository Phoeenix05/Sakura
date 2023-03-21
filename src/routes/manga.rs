use anyhow::Result;
use leptos::*;
use leptos_router::*;
// use reqwasm::http::Headers;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::spawn_local;

use wasm_bindgen;

// async fn fetch_manga_data(uuid: String) -> Result<Value> {
//     let headers = Headers::new();
//     headers.append("Access-Control-Allow-Origi", "tauri://*");

//     let res = reqwasm::http::Request::get(&format!("https://api.mangadex.org/manga/{uuid}/feed"))
//         .headers(headers)
//         .send()
//         .await?
//         .text()
//         .await?;

//     let json: Value = serde_json::from_str(res.as_str()).unwrap();

//     Ok(json)
// }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct MangaDataArgs {
    uuid: String,
}

#[derive(Params, PartialEq, Clone, Debug)]
pub struct MangaParams {
    uuid: String,
}

#[component]
pub fn Manga(cx: Scope) -> impl IntoView {
    // use_router(cx);

    let params = use_params::<MangaParams>(cx);
    let data = create_resource(
        cx,
        move || params.get().map(|p| p.uuid).ok(),
        move |uuid| async move {
            let args = to_value(&MangaDataArgs {
                uuid: uuid.unwrap(),
            });
            let data = invoke("fetch_manga_data", args.unwrap()).await;
            data.as_string().unwrap()
        },
    );

    create_effect(cx, move |_| {
        log!("params = {:#?}", params.get());
    });

    let data_display = move || {
        // data.read(cx).
        // view! { cx, <pre>{  }</pre>}
        match data.read(cx) {
            Some(data) => view! { cx, <pre>{ data }</pre>},
            None => view! { cx, <pre>"Loading..."</pre> },
        }
    };

    view! { cx,
        <div>
            <Transition fallback=move || view! { cx, <p>"Loading..."</p> }>
                { data_display }
            </Transition>
        </div>
    }
}
