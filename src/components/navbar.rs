use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
struct Props {}

#[function_component]
pub fn Navigation() -> Html {
    html! {
        <>
            <div id="top__navbar"></div>
            <div id="side__navbar"></div>
        </>
    }
}
