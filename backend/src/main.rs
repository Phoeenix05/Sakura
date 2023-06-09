#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;

#[tauri::command]
async fn fetch(path: String) -> String {
    let url = format!("https://api.mangadex.org/{}", path);

    let client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: None,
        }))
        .build();

    let req = client.get(url);
    #[cfg(debug_assertions)]
    dbg!(&req);

    let res = req.send().await.unwrap().text().await.unwrap();
    res
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch])
        .setup(|_app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                use tauri::Manager;

                let window = _app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
