use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::navbar::Navigation, routes::manga};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/manga/:uuid")]
    Manga { uuid: String },
    #[at("/manga/:manga_uuid/:chapter_uuid")]
    Chapter {
        manga_uuid: String,
        chapter_uuid: String,
    },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Link<Route> to={ Route::Manga { uuid: "e6eb6bd0-0285-4fac-a6da-9bc4234ac1bb".to_string() }}>{ "Manga" }</Link<Route>>
        },
        Route::Manga { uuid } => html! {
            <manga::Page {uuid} />
        },
        Route::Chapter {
            manga_uuid,
            chapter_uuid,
        } => html! {},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Navigation />
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}
