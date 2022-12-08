#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use image;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu, Window, Wry};
use tauri_api::dialog::Response::*;

#[derive(Clone, serde::Serialize)]
struct OpenFileEventArgs {
    path: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn open_file(window: &Window<Wry>) {
    let filter_list: Option<&str> = None::<&str>;
    let file = tauri_api::dialog::select(filter_list, None::<&str>);
    if let Ok(Okay(file)) = file {
        window
            .emit_all("file-open", OpenFileEventArgs { path: file.clone() })
            .unwrap();
        let image = image::io::Reader::open(file)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgb8();
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
