use crate::histogram::update_active_image_histogram;
use crate::state::STATE;
use crate::util;
use crate::util::prepare_new_filename;
use image_proc::modifications::filters::*;
use image_proc::modifications::prelude::*;
use image_proc::modifications::Transformation;
use tauri::Manager;
use tauri_api::dialog::Response::Okay;

type Window = tauri::Window<tauri::Wry>;

#[tauri::command]
pub fn open_file() -> String {
    let filter_list: Option<&str> = None::<&str>;
    let file = tauri_api::dialog::select(filter_list, None::<&str>);
    let mut state = STATE.lock().unwrap();
    if let Ok(Okay(file)) = file {
        match &state.open_image(&file) {
            Ok(_) => file,
            Err(e) => panic!("{}", e),
        }
    } else {
        String::from("")
    }
}

#[tauri::command]
pub fn apply_negative() -> String {
    let negative = Negative {};
    apply_transfrom(&negative)
}

#[tauri::command]
pub fn apply_hflip() -> String {
    let flip = HorizontalFlip {};
    apply_transfrom(&flip)
}

#[tauri::command]
pub fn apply_vflip() -> String {
    let flip = VerticalFlip {};
    apply_transfrom(&flip)
}

#[tauri::command]
pub fn apply_dflip() -> String {
    let flip = DiagonalFlip {};
    apply_transfrom(&flip)
}

#[tauri::command]
pub fn apply_min_filter(width: u32, height: u32) -> String {
    let transform = MinFilter::new(width, height);
    apply_transfrom(&transform)
}

#[tauri::command]
pub fn apply_max_filter(width: u32, height: u32) -> String {
    match gpu::MaxFilterGPU::try_new(width, height) {
        Ok(transform) => apply_transfrom(&transform),
        Err(_) => {
            let transform = MaxFilter::new(width, height);
            apply_transfrom(&transform)
        }
    }
}

#[tauri::command]
pub fn apply_median_filter(width: u32, height: u32) -> String {
    match gpu::MedianFilterGPU::try_new(width, height) {
        Ok(transform) => apply_transfrom(&transform),
        Err(_) => {
            let transform = MedianFilter::new(width, height);
            apply_transfrom(&transform)
        }
    }
}

#[tauri::command]
pub fn apply_gmean_filter(width: u32, height: u32) -> String {
    match gpu::GMeanFilterGPU::try_new(width, height) {
        Ok(transform) => apply_transfrom(&transform),
        Err(_) => {
            let transform = GeometricMeanFilter::new(width, height);
            apply_transfrom(&transform)
        }
    }
}

#[tauri::command]
pub fn apply_raleigh() -> String {
    let transform = RayleighModification::new(0, 255);
    apply_transfrom(&transform)
}

pub fn apply_and_update<F>(command: F, window: &Window)
where
    F: FnOnce() -> String,
{
    let path = command();
    let state = STATE.lock().unwrap();
    window
        .emit_all("active-path-change", util::PathChangeEventArgs { path })
        .unwrap();
    update_active_image_histogram(&state, window);
}

fn apply_transfrom(transform: &impl Transformation) -> String {
    const FILE_PREFIX: &str = "active";
    let mut state = STATE.lock().unwrap();
    if state.active_image.is_none() {
        return "".to_string();
    }
    state.apply_transform(transform);
    let path = prepare_new_filename(&state.temp_dir, FILE_PREFIX);
    state.active_image.as_ref().unwrap().save(&path).unwrap();
    String::from(path.to_str().unwrap())
}
