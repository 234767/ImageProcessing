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

pub fn open_file(window: &Window) {
    let filter_list: Option<&str> = None::<&str>;
    let file = tauri_api::dialog::select(filter_list, None::<&str>);
    let mut state = STATE.lock().unwrap();
    if let Ok(Okay(file)) = file {
        match &state.open_image(&file) {
            Ok(_) => {
                window
                    .emit_all(
                        "active-path-change",
                        util::PathChangeEventArgs { path: file.clone() },
                    )
                    .unwrap();
                update_active_image_histogram(&state, window);
            }
            Err(e) => panic!("{}", e),
        }
    }
}

pub fn apply_negative(window: &Window) {
    let negative = image_proc::modifications::elementary::Negative {};
    apply_transfrom(window, &negative);
}

pub fn apply_hflip(window: &Window) {
    let flip = image_proc::modifications::geometric::HorizontalFlip {};
    apply_transfrom(window, &flip);
}

pub fn apply_vflip(window: &Window) {
    let flip = image_proc::modifications::geometric::VerticalFlip {};
    apply_transfrom(window, &flip);
}

pub fn apply_dflip(window: &Window) {
    let flip = image_proc::modifications::geometric::DiagonalFlip {};
    apply_transfrom(window, &flip);
}

pub fn apply_min_filter(window: &Window, width: u32, height: u32) {
    let transform = MinFilter::new(width, height);
    apply_transfrom(window, &transform);
}

pub fn apply_max_filter(window: &Window, width: u32, height: u32) {
    match gpu::MaxFilterGPU::try_new(width, height) {
        Ok(transform) => apply_transfrom(window, &transform),
        Err(_) => {
            let transform = MaxFilter::new(width, height);
            apply_transfrom(window, &transform);
        }
    }
}

pub fn apply_median_filter(window: &Window, width: u32, height: u32) {
    match gpu::MedianFilterGPU::try_new(width, height) {
        Ok(transform) => apply_transfrom(window, &transform),
        Err(_) => {
            let transform = MedianFilter::new(width, height);
            apply_transfrom(window, &transform);
        }
    }
}

pub fn apply_gmean_filter(window: &Window, width: u32, height: u32) {
    match gpu::GMeanFilterGPU::try_new(width, height) {
        Ok(transform) => apply_transfrom(window, &transform),
        Err(_) => {
            let transform = GeometricMeanFilter::new(width, height);
            apply_transfrom(window, &transform);
        }
    }
}

pub fn apply_raleigh(window: &Window) {
    let transform = RayleighModification::new(0, 255);
    apply_transfrom(window, &transform);
}

fn apply_transfrom(window: &Window, transform: &impl Transformation) {
    const FILE_PREFIX: &str = "active";
    let mut state = STATE.lock().unwrap();
    if state.active_image.is_none() {
        return;
    }
    state.apply_transform(transform);
    let path = prepare_new_filename(&state.temp_dir, FILE_PREFIX);
    update_active_image_histogram(&state, window);
    state.active_image.as_ref().unwrap().save(&path).unwrap();
    let path = String::from(path.to_str().unwrap());
    window
        .emit_all("active-path-change", util::PathChangeEventArgs { path })
        .unwrap();
    update_active_image_histogram(&state, window);
}
