#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
async fn fetch_manga_data(uuid: String) -> String {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("https://api.mangadex.org/manga/{uuid}/feed"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    res
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_manga_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
