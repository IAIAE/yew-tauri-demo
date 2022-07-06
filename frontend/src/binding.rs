use wasm_bindgen::prelude::*;

pub mod model {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Home {
        pub at: f64,
        pub lo: f64,
        pub desc: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct User {
        pub name: String,
        pub age: u8,
        pub address: Option<Home>,
    }
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(f: JsValue) -> Result<JsValue, JsValue>;
   
    #[wasm_bindgen(js_name = getUser, catch)]
    pub async fn getUser() -> Result<JsValue, JsValue>;


    #[wasm_bindgen(js_name = jsSetYewCb, catch)]
    pub fn jsSetYewCb(cb: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = jsRemoveYewCb, catch)]
    pub fn jsRemoveYewCb() -> Result<(), JsValue>;

}
