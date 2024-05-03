// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod window;
mod pocketbase;

fn main() {
    pocketbase::pb_sidecar!(["migrate", "up"]);
    pocketbase::pb_sidecar!(["admin", "create", "admin@aemil.dev", "password"]);
    pocketbase::pb_sidecar!(["serve"]);

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            crate::window::macos::setup_mac_window(app);
            #[cfg(target_os = "windows")]
            crate::window::windows::setup_win_window(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
