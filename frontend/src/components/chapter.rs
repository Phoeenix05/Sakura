use leptos::{component, view, IntoView, Scope};
use leptos_router::A;

#[derive(Debug)]
pub struct ChapterListItem {
    pub title: String,
    pub number: String,
    pub uuid: uuid::Uuid,
}

#[component]
pub fn ChapterListItem(cx: Scope, props: ChapterListItem) -> impl IntoView {
    let id = format!("chapter {}", props.uuid);
    let href = format!("/chapter/{}", props.uuid);

    view! { cx,
        <A href id>
            <p>{ props.number }" "{ props.title }</p>
        </A>
    }
}
