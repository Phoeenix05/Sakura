use leptos::*;
use leptos_router::*;
use uuid::Uuid;

use mangadex_api::json::manga::MangaFeed;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct MangaParams {
    uuid: Uuid,
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
            let client = reqwest::Client::new();
            let res = client
                .get(format!("https://api.mangadex.org/manga/{uuid}/feed"))
                .send()
                .await
                .unwrap()
                .json::<MangaFeed>()
                .await
                .unwrap();
            res
        },
    );

    create_effect(cx, move |_| {
        log!("params = {:#?}", params.get().unwrap());
    });

    let data_display = move || match data.read(cx) {
        Some(data) => Some(
            data.data
                .into_iter()
                .map(|manga| {
                    let id = manga.id;
                    let title = manga.attributes.title.unwrap_or("null".to_string());
                    view! { cx,
                        <>
                            <A href={"/chapter/".to_owned() + id.to_string().as_str()}>{ title }</A>
                            <br/>
                        </>
                    }
                })
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
