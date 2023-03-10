use gloo::events::EventListener;
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component]
fn SearchBar() -> Html {
    let div_node_ref = use_node_ref();

    use_effect_with_deps(
        {
            let div_node_ref = div_node_ref.clone();

            move |_| {
                let mut key_listener = None;

                if let Some(element) = div_node_ref.cast::<HtmlElement>() {
                    let onkeypress = Callback::from(move |event: Event| {});

                    let listener = EventListener::new(&element, "keypress", move |e| {
                        onkeypress.emit(e.clone())
                    });

                    key_listener = Some(listener);
                }

                move || drop(key_listener)
            }
        },
        div_node_ref.clone(),
    );

    html! {
        <div ref={div_node_ref}></div>
    }
}
