// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use crate::commands::*;
use crate::state::SakuraState;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate cocoa;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod commands;
#[cfg(target_os = "macos")]
mod mac;
mod state;
#[cfg(target_os = "windows")]
mod win;

fn main() {
    let app_state = tauri::async_runtime::block_on(async { SakuraState::init().await });

    tauri::Builder::default()
        .manage(app_state)
        .setup(|app| {
            #[cfg(target_os = "macos")]
            crate::mac::window::setup_mac_window(app);
            #[cfg(target_os = "windows")]
            crate::win::window::setup_win_window(app);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
