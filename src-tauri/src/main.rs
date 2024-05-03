// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod pocketbase;
mod window;

fn main() {
    pocketbase::pb_sidecar!(["migrate", "up"]);
    // this fails after the first run, but it doesn't matter
    pocketbase::pb_sidecar!(["admin", "create", "admin@aemil.dev", "password"]);
    // change the port from 8090 to something more random
    pocketbase::pb_sidecar!(["serve", "--http", "127.0.0.1:11597"]);

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
