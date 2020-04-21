use yew::prelude::*;
use yew::{html, Callback, MouseEvent, Properties};
use yewtil::{Pure, PureComponent};

pub type Button = Pure<PureButton>;

#[derive(Clone, PartialEq, Properties)]
pub struct PureButton {
    pub onsignal: Callback<MouseEvent>,
    pub title: String,
}

impl PureComponent for PureButton {
    fn render(&self) -> Html {
        html! {
            <button onclick=&self.onsignal>{ &self.title }</button>
        }
    }
}
