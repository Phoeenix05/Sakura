use leptos::*;
use leptos_router::*;
use mangadex_api::json::manga::MangaFeed;
use crate::cmd::*;

#[derive(Debug, PartialEq, Params)]
pub struct MangaParams {
    id: uuid::Uuid,
}

#[component]
pub fn Manga(cx: Scope) -> impl IntoView {
    let query = use_params::<MangaParams>(cx);
    
    let uuid = move || {
        query.with(|query| query.as_ref().map(|q| q.id).unwrap_or_default())
    };
    
    let data = create_resource(
        cx,
        move || uuid(),
        move |uuid| async move {
            let path = format!("manga/{}/feed", uuid);
            let json: MangaFeed = fetch(path).await.unwrap();
            json
        }
    );
    
    #[cfg(debug_assertions)]
    create_effect(cx, move |_| {
        log!("{:#?}", uuid())
    });
    
    let data_display = move || match data.read(cx) {
        Some(data) => Some(
            data
                .data
                .into_iter()
                .map(|chapter| view ! { cx,
                    <>
                        <A href=format!("/chapter/{}", chapter.id.to_string())>
                            { chapter.attributes.title.unwrap_or("".to_owned()) }
                        </A>
                        <br />
                    </>
                })
                .collect::<Vec<_>>()
            ),
        None => None,
    };
    let fallback = move || view! { cx, <>"Loading..."</> };
    
    view! { cx,
        <Transition fallback>
            { data_display }
        </Transition>
    }
}
