use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::Navigation, routes::*};

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
    #[at("/settings")]
    Settings,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Link<Route> to={ Route::Manga { uuid: "e6eb6bd0-0285-4fac-a6da-9bc4234ac1bb".to_string() }}>{ "Manga" }</Link<Route>>
        },
        Route::Manga { uuid } => html! {
            <MangaPage {uuid} />
        },
        Route::Chapter { .. } => html! {},
        Route::Settings => html! {
            <SettingsPage />
        },
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
