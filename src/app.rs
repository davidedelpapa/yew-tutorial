//use crate::external::ExternalService;
use crate::components::button::Button;
use yew::prelude::*;

pub enum Msg {
    AddOne,
    RemoveOne,
}

pub struct App {
    link: ComponentLink<Self>,
    counter: i32,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link: link,
            counter: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.counter += 1;
            }
            Msg::RemoveOne => {
                self.counter -= if self.counter == 0 { 0 } else { 1 };
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "Welcome to Components" }</h1>
                <p>{ self.counter } </p>
                <Button onsignal=self.link.callback(|_| Msg::AddOne) title="+1" />
                <Button onsignal=self.link.callback(|_| Msg::RemoveOne) title="-1" />
            </div>
        }
    }
}
