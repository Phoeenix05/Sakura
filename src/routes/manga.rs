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
    // let abort_controller = web_sys::AbortController::new().ok();
    // let abort_signal = abort_controller.as_ref().map(|a| a.signal()).unwrap();

    let params = use_params_map(cx);

    let data = create_resource(
        cx,
        move || {
            params.get().get("uuid").unwrap().clone()
            // abort_signal.clone(),
        },
        move |uuid| async move {
            // NOTE: For some reason querying the data inside this functions throws an error
            // so for now it is done on tauri's side. Also cleaning up data when navigating off of
            // this page isn't done properly.
            let data = invoke("fetch_manga_data", to_value(&MangaParams { uuid }).unwrap())
                .await
                .as_string()
                .unwrap();
            data
        },
    );

    create_effect(cx, move |_| {
        log!("params = {:#?}", params.get());
    });

    on_cleanup(cx, move || {
        // if let Some(abort_controller) = abort_controller {
        //     abort_controller.abort();
        // }
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
