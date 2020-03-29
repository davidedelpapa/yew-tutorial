use rand::prelude::*;
use yew::prelude::*;

pub enum Msg {
    AddOne,
    RemoveOne,
}

pub struct App {
    items: Vec<i64>,
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            items: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.items.push(random()),
            Msg::RemoveOne => {
                self.items.pop();
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <p> {"Items: "} { format!("{:?}", self.items) }</p>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add 1" }</button>
                <button onclick=self.link.callback(|_| Msg::RemoveOne)>{ "Remove 1" }</button>
            </div>
        }
    }
}
