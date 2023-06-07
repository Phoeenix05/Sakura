use leptos::*;

mod cmd;
mod components;
mod routes;

use routes::*;

pub fn main() {
    mount_to_body(|cx| view! { cx, <Pages /> })
}
