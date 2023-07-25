use leptos::*;
use leptos_router::*;
use mangadex_api::json::types::*;

use crate::cmd::*;
use crate::components::chapter::*;
use crate::components::manga::*;

#[derive(Debug, PartialEq, Params)]
pub struct MangaParams {
    id: uuid::Uuid,
}

#[component]
pub fn Manga(cx: Scope) -> impl IntoView {
    let params = use_params::<MangaParams>(cx);

    let uuid = move || params.with(|query| query.as_ref().map(|q| q.id).unwrap_or_default());

    let feed = create_resource(cx, uuid, move |uuid| async move {
        let query = "order[chapter]=desc&limit=500&translatedLanguage[]=en";
        let path = format!("manga/{}/feed?{}", uuid, query);
        let json: MangaFeed = fetch(path).await.unwrap();
        json
    });
    let _info = create_resource(cx, uuid, move |uuid| async move {
        let path = format!("manga/{}", uuid);
        let json: Manga = fetch(path).await.unwrap();
        json
    });

    #[cfg(debug_assertions)]
    create_effect(cx, move |_| log!("{:#?}", uuid()));

    let feed_display = move || match feed.read(cx) {
        Some(feed) => Some(
            feed.data
                .into_iter()
                .map(|chapter| {
                    view! { cx,
                        <div>
                            <MangaBanner props=MangaBanner { uuid: uuid() } />
                            <ChapterListItem props=ChapterListItem {
                                title: chapter.attributes.title.unwrap_or("".into()),
                                number: chapter.attributes.chapter,
                                uuid: chapter.uuid,
                            } />
                        </div>
                    }
                })
                .collect::<Vec<_>>(),
        ),
        None => None,
    };
    let fallback = move || view! { cx, <>"Loading..."</> };

    view! { cx,
        <Transition fallback>
            { feed_display }
        </Transition>
    }
}
