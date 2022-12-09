#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod histogram;
mod menu;
mod state;
mod util;
mod commands;

use commands::*;

fn main() {
    let builder = tauri::Builder::default();
    let builder = menu::set_menu(builder);
    let builder = menu::setup_menu_events(builder);
    builder
        .invoke_handler(tauri::generate_handler![open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
