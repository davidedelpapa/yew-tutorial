use crate::api::{HostipService, ServerResponse};
use crate::external::ExternalService;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;

pub enum Msg {
    PollService,
    GetIpResponse,
    IpResponseReady(Result<ServerResponse, Error>),
}

pub struct App {
    link: ComponentLink<Self>,
    service: ExternalService,
    uuidv4: String,
    ipservice: HostipService,
    r: Option<ServerResponse>,
    callback: Callback<Result<ServerResponse, Error>>,
    task: Option<FetchTask>,
    console: ConsoleService,
    ip: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link: link.clone(), // watchout!
            service: ExternalService::new(),
            uuidv4: "".to_string(),
            ipservice: HostipService::new(),
            callback: link.callback(Msg::IpResponseReady),
            r: None,
            task: None,
            console: ConsoleService::new(),
            ip: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PollService => {
                self.uuidv4 = self.service.uuidv4();
            }
            Msg::GetIpResponse => {
                let task = self.ipservice.get_response(self.callback.clone());
                self.task = Some(task);
            }
            Msg::IpResponseReady(Ok(r)) => {
                self.r = Some(r);
                self.console
                    .log(format!("Response: {:?}", Json(self.r.clone())).as_str());
                self.ip = self.r.as_mut().unwrap().ip.clone();
            }
            Msg::IpResponseReady(Err(e)) => {
                self.console.log(format!("Error: {:?}", e).as_str());
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ format!("{}", self.ip ) }</h1>
                <button onclick=self.link.callback(|_| Msg::PollService)>{ "Get a UUID" }</button>
                <button onclick=self.link.callback(|_| Msg::GetIpResponse)>{ "Get my IP" }</button>
                <p>{ format!("{}", &self.uuidv4) }</p>
            </div>
        }
    }
}
