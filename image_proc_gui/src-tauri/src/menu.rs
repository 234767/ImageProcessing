use crate::commands::*;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

type Builder = tauri::Builder<tauri::Wry>;

pub fn set_menu(builder: Builder) -> Builder {
    let file_submenu = create_file_submenu();
    let image_submenu = create_image_submenu();
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::Quit)
        .add_submenu(file_submenu)
        .add_submenu(image_submenu);

    builder.menu(menu)
}

fn create_file_submenu() -> Submenu {
    let quit = CustomMenuItem::new("quit", "Quit");
    let open = CustomMenuItem::new("open", "Open");
    let submenu = Menu::new().add_item(quit).add_item(open);

    Submenu::new("File", submenu)
}

fn create_image_submenu() -> Submenu {
    let invert = CustomMenuItem::new("negative", "Invert");
    let submenu = Menu::new()
        .add_item(invert)
        .add_submenu(create_transform_submenu());

    Submenu::new("Image", submenu)
}

fn create_transform_submenu() -> Submenu {
    let hflip = CustomMenuItem::new("hflip", "Flip horizontally");
    let vflip = CustomMenuItem::new("vflip", "Flip vertically");
    let dflip = CustomMenuItem::new("dflip", "Flip diagonally");
    let submenu = Menu::new().add_item(hflip).add_item(vflip).add_item(dflip);

    Submenu::new("Transform", submenu)
}

pub fn setup_menu_events(builder: Builder) -> Builder {
    builder.on_menu_event(|event| {
        let window = event.window();
        match event.menu_item_id() {
            "quit" => std::process::exit(0),
            "open" => open_file(window),
            "negative" => apply_negative(window),
            "hflip" => apply_hflip(window),
            "vflip" => apply_vflip(window),
            "dflip" => apply_dflip(window),
            _ => {}
        }
    })
}
