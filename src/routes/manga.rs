use yew::prelude::*;
use yew_router::prelude::Link;

use crate::components::app::Route;

#[derive(PartialEq, Properties)]
struct Props {
    uuid: String
}

#[function_component]
fn Page(props: &Props) -> Html {
    html! {
        <div>
            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            { props.uuid.clone() }
        </div>
    }
}
