use yew::{function_component, html, Html, Properties};
use yew_router::prelude::Link;

use crate::app::Route;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component]
pub fn Navigation(_props: &Props) -> Html {
    html! {
        <div id="navbar">
            <div id="top"></div>
            <div id="bottom">
                <div class="nav_item" id="settings">
                    <a href="/settings">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="white" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 13.5V3.75m0 9.75a1.5 1.5 0 010 3m0-3a1.5 1.5 0 000 3m0 3.75V16.5m12-3V3.75m0 9.75a1.5 1.5 0 010 3m0-3a1.5 1.5 0 000 3m0 3.75V16.5m-6-9V3.75m0 3.75a1.5 1.5 0 010 3m0-3a1.5 1.5 0 000 3m0 9.75V10.5" />
                        </svg>
                    </a>
                </div>
            </div>
        </div>
    }
}
