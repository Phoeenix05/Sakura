use yew::{function_component, html, Callback, Html, Properties};
use yew_router::prelude::use_navigator;

use crate::app::Route;

#[function_component]
fn SettingsIcon() -> Html {
    html! {
        <svg aria-label="settings icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="white" class="w-6 h-6">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 13.5V3.75m0 9.75a1.5 1.5 0 010 3m0-3a1.5 1.5 0 000 3m0 3.75V16.5m12-3V3.75m0 9.75a1.5 1.5 0 010 3m0-3a1.5 1.5 0 000 3m0 3.75V16.5m-6-9V3.75m0 3.75a1.5 1.5 0 010 3m0-3a1.5 1.5 0 000 3m0 9.75V10.5" />
        </svg>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component]
pub fn Navigation(_props: &Props) -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Settings));
    html! {
        <div id="navbar">
            <div id="top"></div>
            <div id="bottom">
                <button {onclick} class="nav_item" id="settings">
                    <SettingsIcon />
                </button>
            </div>
        </div>
    }
}
