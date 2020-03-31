use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::StorageService;

const KEY: &'static str = "yew.tut.database";

#[wasm_bindgen(
    inline_js = "export function refreshform(form) { document.getElementById(form).reset(); }"
)]
extern "C" {
    fn refreshform(form: &str);
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    tasks: Vec<Task>,
}

impl Database {
    pub fn new() -> Self {
        Database { tasks: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    title: String,
    description: String,
}

impl Task {
    pub fn new() -> Self {
        Task {
            title: "".to_string(),
            description: "".to_string(),
        }
    }
    pub fn is_filledin(&self) -> bool {
        (self.title != "") && (self.description != "")
    }
}

pub enum Msg {
    AddTask,
    RemoveTask(usize),
    SetTitle(String),
    SetDescription(String),
}

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    database: Database,
    temp_task: Task,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local);
        let Json(database) = storage.restore(KEY);
        let database = database.unwrap_or_else(|_| Database::new());
        App {
            link,
            storage,
            database,
            temp_task: Task::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTask => {
                if self.temp_task.is_filledin() {
                    self.database.tasks.push(self.temp_task.clone());
                    self.storage.store(KEY, Json(&self.database));
                    self.temp_task = Task::new();
                    refreshform("taskform");
                }
            }
            Msg::RemoveTask(pos) => {
                let _ = self.database.tasks.remove(pos);
                self.storage.store(KEY, Json(&self.database));
            }
            Msg::SetTitle(title) => {
                self.temp_task.title = title;
            }
            Msg::SetDescription(description) => {
                self.temp_task.description = description;
            }
        }
        true
    }

    fn view(&self) -> Html {
        let render_item = |(idx, task): (usize, &Task)| {
            html! {
                <>
                    <div class="card">
                        <header><label>{ &task.title }</label></header>
                        <div class="card-body"><label>{ &task.description }</label></div>
                        <footer><button onclick=self.link.callback(move |_| Msg::RemoveTask(idx))>{ "Remove" }</button></footer>
                    </div>
                </>
            }
        };
        html! {
            <div>
                <h2>{"Tasks: "}</h2>
                { for self.database.tasks.iter().enumerate().map(render_item) }
                <div class="card">
                    <form id="taskform">
                        <label class="stack"><input placeholder="Title" oninput=self.link.callback(|e: InputData|  Msg::SetTitle(e.value)) /></label>
                        <label class="stack"><textarea rows=2 placeholder="Description" oninput=self.link.callback(|e: InputData|  Msg::SetDescription(e.value))></textarea></label>
                        <button class="stack icon-paper-plane" onclick=self.link.callback(|_|  Msg::AddTask)>{ "Add task" }</button>
                    </form>
                </div>
            </div>
        }
    }
}
