use anyhow::Result;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_wasm_bindgen::to_value;

use crate::json::MangaFeed;
use crate::util::invoke;

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
            let chapters = json
                .data
                .into_iter()
                .map(|chapter| (chapter.attributes.title, chapter.id))
                .collect::<Vec<_>>();
            chapters
        },
    );

    create_effect(cx, move |_| {
        log!("params = {:#?}", params.get().unwrap());
    });

    let data_display = move || match data.read(cx) {
        Some(data) => Some(
            data.into_iter()
                .map(|(_title, id)| view! { cx, <p>{ id }</p> })
                .collect::<Vec<_>>(),
        ),
        None => None,
    };

    view! { cx,
        <div id="manga_data">
            <Transition fallback=move || view! { cx, <>"Loading..."</>}>
                { data_display }
            </Transition>
        </div>
    }
}
