use anyhow::Result;
use leptos::*;
use leptos_router::*;
// use serde::{Deserialize, Serialize};s
use serde_json::Value;
// use wasm_bindgen_futures::spawn_local;

async fn fetch_manga_data(uuid: String) -> Result<Value> {
    let res = reqwasm::http::Request::get(&format!("https://api.mangadex.org/manga/{uuid}/feed"))
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(res.as_str()).unwrap();

    Ok(json)
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
        move |uuid| async move { fetch_manga_data(uuid.unwrap()).await.unwrap() },
    );

    create_effect(cx, move |_| {
        log!("params = {:#?}", params.get());
    });

    let data_display = move || {
        // data.read(cx).
        // view! { cx, <pre>{  }</pre>}
        match data.read(cx) {
            Some(data) => view! { cx, <pre>{ data.to_string() }</pre>},
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
