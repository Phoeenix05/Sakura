use leptos::*;
use leptos_router::*;
use mangadex_api::json::types::*;

use crate::cmd::*;

#[derive(Debug, PartialEq, Params)]
pub struct ChapterParams {
    id: uuid::Uuid,
}

// #[derive(Debug, PartialEq, Params)]
// pub struct ChapterQueries {
//     prev_chapter: Option<uuid::Uuid>,
//     next_chapter: Option<uuid::Uuid>,
// }

#[component]
pub fn Chapter(cx: Scope) -> impl IntoView {
    let params = use_params::<ChapterParams>(cx);

    let uuid = move || params.with(|param| param.as_ref().map(|p| p.id).unwrap_or_default());

    let image_server = create_resource(cx, uuid, move |uuid| async move {
        let path = format!("at-home/server/{}", uuid);
        let json: AtHomeServer = fetch(path).await.unwrap();
        json
    });
    // let manga_feed = create_resource(cx, source, fetcher)

    #[cfg(debug_assertions)]
    create_effect(cx, move |_| log!("{:#?}", uuid()));

    let data_display = move || match image_server.read(cx) {
        Some(data) => {
            let base_url = data.base_url;
            let access_token = data.data.hash;
            let url = format!("{base_url}/data/{access_token}");

            let images = if data.data.data.len() > 0 {
                data.data.data
            } else {
                data.data.data_saver
            };

            Some(
                images
                    .into_iter()
                    .map(|image| view! { cx, <img src=format!("{url}/{image}") /> })
                    .collect::<Vec<_>>(),
            )
        }
        None => None,
    };
    let fallback = move || view! { cx, <>"Loading..."</> };

    view! { cx,
        <Transition fallback>
            <div id="chapter_container">
                { data_display }
            </div>
        </Transition>
    }
}
