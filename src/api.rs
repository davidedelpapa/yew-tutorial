use anyhow::{anyhow, Error};
use serde::Deserialize;

use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Deserialize, Debug, Clone)]
pub struct ServerResponse {
    pub country_name: String,
    pub country_code: String,
    pub city: String,
    pub ip: String,
}

#[derive(Default)]
pub struct HostipService {
    fetchserv: FetchService,
}

impl HostipService {
    pub fn new() -> Self {
        Self {
            fetchserv: FetchService::new(),
        }
    }

    pub fn get_response(&mut self, callback: Callback<Result<ServerResponse, Error>>) -> FetchTask {
        let url = format!("http://api.hostip.info/get_json.php");
        let handler = move |response: Response<Json<Result<ServerResponse, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(anyhow!(
                    "{}: error getting ip from http://api.hostip.info/get_json.php",
                    meta.status
                )))
            }
        };
        let request = Request::get(url.as_str()).body(Nothing).unwrap();
        self.fetchserv.fetch(request, handler.into())
    }
}
