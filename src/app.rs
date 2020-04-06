use crate::external::ExternalService;
use yew::prelude::*;

pub enum Msg {
    PollService,
}

pub struct App {
    link: ComponentLink<Self>,
    service: ExternalService,
    uuidv4: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            service: ExternalService::new(),
            uuidv4: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PollService => {
                self.uuidv4 = self.service.uuidv4();
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::PollService)>{ "Get a UUID" }</button>
                <p>{ format!("{}", &self.uuidv4) }</p>
            </div>
        }
    }
}
