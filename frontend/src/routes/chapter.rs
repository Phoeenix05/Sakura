use leptos::*;
use leptos_router::*;
use mangadex_api::json::at_home::AtHomeServer;
use crate::cmd::*;

#[derive(Debug, PartialEq, Params)]
pub struct ChapterParams {
    id: uuid::Uuid,
}

#[component]
pub fn Chapter(cx: Scope) -> impl IntoView {
    let query = use_params::<ChapterParams>(cx);

    let uuid = move || {
        query.with(|query| query.as_ref().map(|q| q.id).unwrap_or_default())
    };

    let data = create_resource(
        cx,
        move || uuid(),
        move |uuid| async move {
            let path = format!("at-home/server/{}", uuid);
            let json: AtHomeServer = fetch(path).await.unwrap();
            json
        }
    );

    #[cfg(debug_assertions)]
    create_effect(cx, move |_| {
        log!("{:#?}", uuid())
    });

    let data_display = move || match data.read(cx) {
        Some(data) => {
            let base_url = data.base_url;
            let access_token = data.chapter.hash;
            let url = format!("{base_url}/data/{access_token}");
            Some(data
                .chapter
                .data
                .into_iter()
                .map(|image| view! { cx, <img src=format!("{url}/{image}") /> })
                .collect::<Vec<_>>()
            )
        },
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
