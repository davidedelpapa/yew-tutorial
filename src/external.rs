use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Default)]
pub struct ExternalService();

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn uuidv4() -> JsValue;
}

impl ExternalService {
    pub fn new() -> Self {
        ExternalService()
    }

    pub fn uuidv4(&mut self) ->String {
        let v = uuidv4();
        v.as_string()
        .unwrap_or_else(|| {
            console::log_1(&JsValue::from_str("Can't get a uuid")); 
            "".to_string()
        })
    }
}
