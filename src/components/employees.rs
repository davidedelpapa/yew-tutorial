use crate::components::api::{ApiResponse, Employee, Request};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewtil::fetch::{Fetch, FetchAction, FetchState};
use yewtil::future::LinkFuture;

pub struct Employees {
    api: Fetch<Request, ApiResponse>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse>),
    GetApi,
}

impl Component for Employees {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Employees {
            api: Default::default(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetApiFetchState(fetch_state) => {
                self.api.apply(fetch_state);
                true
            }
            Msg::GetApi => {
                self.link.send_future(self.api.fetch(Msg::SetApiFetchState));
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.api.as_ref().state() {
            FetchState::NotFetching(_) => {
                html! {<button onclick=self.link.callback(|_| Msg::GetApi)>{"Get employees"}</button>}
            }
            FetchState::Fetching(_) => html! {"Fetching"},
            FetchState::Fetched(data) => {
                if data.status != "success".to_string() {
                    return html! {"Api Error!"};
                }
                data.data
                    .iter()
                    .map(|e: &Employee| {
                        html! {
                            <div>
                                <div>
                                    <p>
                                        {"Name: "}
                                        {&e.employee_name}
                                    </p>
                                    <p>
                                        {"Salary: "}
                                        {&e.employee_salary}
                                    </p>
                                    <p>
                                        {"Age: "}
                                        {&e.employee_age}
                                    </p>
                                </div>
                                <hr />
                            </div>
                        }
                    })
                    .collect()
            }
            FetchState::Failed(_, err) => html! {&err},
        }
    }
}
