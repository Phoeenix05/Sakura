use anyhow::Result;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use mangadex_api::json;
use mangadex_api::wrapper::get_manga_feed;

use uuid::Uuid;

#[derive(Params, PartialEq, Clone, Debug, Serialize, Deserialize)]
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
            let res = get_manga_feed(uuid, None).await;

            match res {
                json::Result::Ok(res) => Ok(res),
                json::Result::Err(err) => Err(err)
            }
        },
    );

    create_effect(cx, move |_| {
        log!("params = {:#?}", params.get().unwrap());
    });

    let data_display = move || {
        match data.read(cx) {
        Some(data) => Some(
            data.unwrap().data.into_iter()
                .map(|manga| view! { cx, <><A href={"/chapter/".to_owned() + &manga.id.to_string().as_str()}>{ manga.attributes.title.unwrap() }</A><br/></> })
                .collect::<Vec<_>>()
        ),
        None => None,
    }
    };

    view! { cx,
        <div id="manga_data">
            <Transition fallback=move || view! { cx, <>"Loading..."</>}>
                { data_display }
            </Transition>
        </div>
    }
}
