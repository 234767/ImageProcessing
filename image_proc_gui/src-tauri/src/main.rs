#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::histogram::update_active_image_histogram;
use crate::state::STATE;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu, Window, Wry};
use tauri_api::dialog::Response::*;

mod events;
mod histogram;
mod state;

fn open_file(window: &Window<Wry>) {
    let filter_list: Option<&str> = None::<&str>;
    let file = tauri_api::dialog::select(filter_list, None::<&str>);
    let mut state = STATE.lock().unwrap();
    if let Ok(Okay(file)) = file {
        match &state.open_image(&file) {
            Ok(_) => {
                window
                    .emit_all(
                        "file-open",
                        events::PathChangeEventArgs { path: file.clone() },
                    )
                    .unwrap();
                update_active_image_histogram(&state, window);
            }
            Err(e) => panic!("{}", e),
        }
    }
}

fn main() {
    let quit = CustomMenuItem::new("quit", "Quit");
    let open = CustomMenuItem::new("open", "Open");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(open));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => std::process::exit(0),
            "open" => open_file(event.window()),
            _ => {}
        })
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
