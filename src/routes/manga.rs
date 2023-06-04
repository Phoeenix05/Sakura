use leptos::*;
use leptos_router::*;
use tauri::api::http::HttpRequestBuilder;
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
            let url = format!("https://api.mangadex.org/manga/{uuid}/feed");
            let client = tauri::api::http::ClientBuilder::new().build().unwrap();
            let res = client
                .send(
                    HttpRequestBuilder::new("GET", url)
                        .unwrap()
                        .response_type(tauri::api::http::ResponseType::Text),
                )
                .await;

            let json: MangaFeed = serde_json::from_value(res.unwrap().read().await.unwrap().data).unwrap();
            json
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
