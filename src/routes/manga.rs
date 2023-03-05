use yew::prelude::*;
use yew_router::prelude::Link;

use crate::components::app::Route;

// #[derive(PartialEq, Properties)]
// pub struct Props {
//     pub uuid: String
// }

// #[function_component]
// pub fn Page(props: &Props) -> Html {
//     html! {
//         <div>
//             <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
//             { props.uuid.clone() }
//         </div>
//     }
// }

// fn fetch_data() {
//     todo!()
// }

pub enum Msg {
    DataLoaded(String),
    Error,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub uuid: String,
}

pub struct Page {
    // data: Option<String>,
    uuid: String,
}

impl Component for Page {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            uuid: _ctx.props().uuid.clone(),
            // data: Some("".to_string()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DataLoaded(data) => {
                
                true
            },
            Msg::Error => {
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{ &self.uuid }</p>
                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            </>
        }
    }
}
