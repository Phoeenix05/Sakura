use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn fetch<T>(path: String) -> Result<T, serde_json::Error>
where
    T: serde::de::DeserializeOwned,
{
    let args = serde_wasm_bindgen::to_value(&FetchArgs { path }).unwrap();
    let res = invoke("fetch", args).await.as_string().unwrap();
    let json: T = serde_json::from_str(&res).unwrap();
    Ok(json)
}

#[derive(serde::Serialize)]
pub struct FetchArgs {
    pub path: String,
}
