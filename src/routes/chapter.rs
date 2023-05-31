use leptos::*;
use leptos_router::*;

use mangadex_api::json::at_home::AtHomeServer;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct ChapterParams {
    uuid: String,
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
            let client = reqwest::Client::new();
            let res = client
                .get(format!("https://api.mangadex.org/at-home/server/{uuid}"))
                .send()
                .await
                .unwrap()
                .json::<AtHomeServer>()
                // .text()
                .await
                .unwrap();
            res
        },
    );

    create_effect(cx, move |_| {
        log!("params = {:#?}", params.get().unwrap());
    });

    let data_display = move || match data.read(cx) {
        Some(data) => view! { cx, <><pre>{ format!("{data:#?}") }</pre></> },
        None => view! { cx, <></> },
    };

    view! { cx,
        <Transition fallback=move || view! { cx, <>"Loading..."</> }>
            { data_display }
        </Transition>
    }
}
