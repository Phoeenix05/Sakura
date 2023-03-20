use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct MangaParams {
    uuid: String,
}

#[component]
pub fn Manga(cx: Scope) -> impl IntoView {
    // Get page params
    let params = use_params::<MangaParams>(cx);
    let _data = create_resource(
        cx,
        move || params.get().map(|params| params.uuid).ok().unwrap(),
        move |_| async move {
            spawn_local(async {
                // let client = reqwest::Client::new();
                // let res = client.get("https://api.mangadex.org/list/e81b4d3d-2692-4af9-9300-91012d079cd6/feed").send().await.unwrap().text().await;
                // log!("{:#?}", res);
            })
        },
    );

    view! { cx,
        <main></main>
    }
}
