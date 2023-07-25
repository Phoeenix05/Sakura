use leptos::*;
use leptos_router::*;
use mangadex_api::json::types::{Cover, Manga};
use uuid::Uuid;

use crate::cmd::fetch;

pub struct MangaBanner {
    pub uuid: Uuid,
}

pub struct MangaInfo {
    pub uuid: Uuid,
}

#[component]
pub fn MangaBanner(cx: Scope, props: MangaBanner) -> impl IntoView {
    let manga_data = create_resource(
        cx,
        move || props.uuid.clone(),
        move |uuid| async move {
            let path = format!("manga/{}", uuid);
            let json: Manga = fetch(path).await.unwrap();
            json
        },
    );

    let cover_art_uuid = move || {
        let info = match manga_data.read(cx) {
            Some(info) => info,
            None => return None,
        };

        let cover_art_uuid = info
            .data
            .relationships
            .into_iter()
            .find(|e| e.data_type == "cover_art")
            .unwrap()
            .uuid;

        Some(cover_art_uuid)
    };

    let art_data = create_resource(cx, cover_art_uuid, move |cover_uuid| async move {
        let path = format!("cover/{}", cover_uuid.unwrap());
        let json: Cover = fetch(path).await.unwrap();
        json
    });

    let cover_url = match art_data.read(cx) {
        Some(data) => {
            let file_name = data.data.attributes.file_name;
            let url = format!("https://uploads.mangadex.org/{}/{}", props.uuid, file_name);
            Some(url)
        }
        None => None,
    }
    .unwrap();

    let fallback = move || view! { cx, <>"Loading..."</> };

    view! { cx,
        <Transition fallback>
            <img src={cover_url.clone()} alt="Cover Art" />
        </Transition>
    }
}

#[component]
pub fn MangaInfo(cx: Scope, props: MangaInfo) -> impl IntoView {
    view! { cx, <></> }
}
