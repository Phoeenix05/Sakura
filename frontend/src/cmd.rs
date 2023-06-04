use wasm_bindgen;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use serde::de::DeserializeOwned;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn fetch<T>(path: String) -> Result<T, serde_json::Error> where T: DeserializeOwned {
    let args = to_value(&FetchArgs { path }).unwrap();
    let res = invoke("fetch", args).await.as_string().unwrap();
    let json: T = serde_json::from_str(&res).unwrap();
    Ok(json)
}

#[derive(serde::Serialize)]
pub struct FetchArgs {
    pub path: String,
}
