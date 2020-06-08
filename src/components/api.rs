use serde::{Deserialize, Serialize};
use yewtil::fetch::{FetchRequest, Json, MethodBody};

const API_URL: &'static str = "http://dummy.restapiexample.com/api/v1/employees";

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Employee {
    pub id: String,
    pub employee_name: String,
    pub employee_salary: String,
    pub employee_age: String,
    pub profile_image: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse {
    pub status: String,
    pub data: Vec<Employee>,
}

#[derive(Default, Debug, Clone)]
pub struct Request;

impl FetchRequest for Request {
    type RequestBody = ();
    type ResponseBody = ApiResponse;
    type Format = Json;

    fn url(&self) -> String {
        API_URL.to_string()
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        vec![]
    }

    fn use_cors(&self) -> bool {
        true
    }
}
