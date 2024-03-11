// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::commands::*;
use crate::state::SakuraState;
use tauri::generate_handler;

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
    tauri::Builder::default()
        .manage(SakuraState::init())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            crate::mac::window::setup_mac_window(app);
            #[cfg(target_os = "windows")]
            crate::win::window::setup_win_window(app);

            Ok(())
        })
        .invoke_handler(generate_handler![construct_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
