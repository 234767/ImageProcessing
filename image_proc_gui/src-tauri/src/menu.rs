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
        .add_submenu(create_transform_submenu())
        .add_submenu(create_filter_submenu())
        .add_submenu(create_blur_submenu())
        .add_submenu(create_histogram_submenu());

    Submenu::new("Image", submenu)
}

fn create_transform_submenu() -> Submenu {
    let hflip = CustomMenuItem::new("hflip", "Flip horizontally");
    let vflip = CustomMenuItem::new("vflip", "Flip vertically");
    let dflip = CustomMenuItem::new("dflip", "Flip diagonally");
    let submenu = Menu::new().add_item(hflip).add_item(vflip).add_item(dflip);

    Submenu::new("Transform", submenu)
}

fn create_filter_submenu() -> Submenu {
    let min = CustomMenuItem::new("min", "Minimum filter");
    let max = CustomMenuItem::new("max", "Maximum filter");
    let median = CustomMenuItem::new("median", "Median filter");
    let gmean = CustomMenuItem::new("gmean", "Geometric mean filter");
    let basic_filters = Menu::new()
        .add_item(min)
        .add_item(max)
        .add_item(median)
        .add_item(gmean);

    let conv = CustomMenuItem::new("convolution", "Convolution linear filter");

    let uolis = CustomMenuItem::new("uolis", "Uolis operator");
    let roberts = CustomMenuItem::new("roberts","Roberts operator");

    let operators_menu = Menu::new().add_item(uolis).add_item(roberts);

    let submenu = Menu::new()
        .add_submenu(Submenu::new("Basic", basic_filters))
        .add_item(conv)
        .add_submenu(Submenu::new("Non-linear operators", operators_menu));

    Submenu::new("Filter", submenu)
}

fn create_blur_submenu() -> Submenu {
    let mean = CustomMenuItem::new("mean", "Linear blur");
    let gaussian_blur = CustomMenuItem::new("gaussian-blur", "Gaussian blur");
    let blur_menu = Menu::new().add_item(mean).add_item(gaussian_blur);

    Submenu::new("Blur", blur_menu)
}

fn create_histogram_submenu() -> Submenu {
    let rayleigh = CustomMenuItem::new("rayleigh", "Rayleigh PDF equalization");
    let submenu = Menu::new().add_item(rayleigh);

    Submenu::new("Histogram", submenu)
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
            "min" => apply_min_filter(window,3,3),
            "max" => apply_max_filter(window,3,3),
            "median" => apply_median_filter(window,3,3),
            "gmean" => apply_gmean_filter(window,3,3),
            "rayleigh" => apply_raleigh(window),
            _ => {}
        }
    })
}
