// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn user_agent(app: tauri::AppHandle) -> String {
    format!("Sakura {} / Tauri", app.package_info().version)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![user_agent])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
