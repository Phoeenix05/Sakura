// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn search_manga(query: String) -> Result<String, ()> {
    Ok("".into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_manga])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
