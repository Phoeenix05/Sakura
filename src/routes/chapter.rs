use leptos::*;
use leptos_router::*;
use uuid::Uuid;

use mangadex_api::json::at_home::AtHomeServer;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct ChapterParams {
    uuid: Uuid,
}

#[component]
pub fn Chapter(cx: Scope) -> impl IntoView {
    let params = use_params::<ChapterParams>(cx);

    let data = create_resource(
        cx,
        move || {
            let uuid = params.get().unwrap().uuid.to_owned();
            uuid
        },
        move |uuid| async move {
            let url = format!("https://api.mangadex.org/at-home/server/{uuid}");
            let client = reqwest::Client::new();
            let res = client.get(url).send().await.unwrap().json::<AtHomeServer>().await.unwrap();
            res
        },
    );

    create_effect(cx, move |_| {
        log!("params = {:#?}", params.get().unwrap());
    });

    let data_display = move || match data.read(cx) {
        Some(data) => {
            let base_url = data.base_url;
            let hash = data.chapter.hash;
            let url = format!("{}/data/{}", base_url, hash);

            let display = data
                .chapter
                .data
                .into_iter()
                .map(|image| {
                    view! { cx,
                       <>
                           <img id="image" src={format!("{}/{}", url, image)} />
                       </>
                    }
                })
                .collect::<Vec<_>>();

            Some(display)
        }
        None => None,
    };

    view! { cx,
        <div id="chapter_data">
            <Transition fallback=move || view! { cx, <>"Loading..."</>}>
                { data_display }
            </Transition>
        </div>
    }
}
