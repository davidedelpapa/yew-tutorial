use yew::{html, Callback, Html, MouseEvent};
use yewtil::function_component;

#[function_component(Button)]
pub fn button(onsignal: &Callback<MouseEvent>, #[prop_or_default] title: String) -> Html {
    html! {
        <button onclick=onsignal>{ title }</button>
    }
}
