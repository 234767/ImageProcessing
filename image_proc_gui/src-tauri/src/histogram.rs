use crate::events;
use crate::state::AppState;
use image::{ImageBuffer, RgbImage};
use image_proc::histogram::Histogram;
use std::fmt::format;
use std::fs;
use std::path::Path;
use tauri::{Manager, Window, Wry};
use uuid::Uuid;

pub fn update_active_image_histogram(state: &AppState, window: &Window<Wry>) {
    const FILE_PREFIX: &str = "histogram-original";
    match &state.active_image {
        None => {}
        Some(img) => {
            for file in fs::read_dir(&state.temp_dir).unwrap() {
                if file
                    .as_ref()
                    .unwrap()
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with(FILE_PREFIX)
                {
                    fs::remove_file(file.unwrap().path()).unwrap();
                }
            }
            let path = &state
                .temp_dir
                .join(format!("{}{}.bmp", FILE_PREFIX, Uuid::new_v4()));
            create_histogram(img, path.as_path());
            window
                .emit_all(
                    "active-histogram-update",
                    events::PathChangeEventArgs {
                        path: String::from(path.to_str().unwrap()),
                    },
                )
                .unwrap();
        }
    }
}

pub fn create_histogram(image: &RgbImage, path: &Path) {
    const HISTOGRAM_HEIGHT: u32 = 100;

    let normalized_histogram = {
        let mut histogram = Histogram::new(image);
        let max_height = *histogram.iter().flat_map(|h| h.iter()).max().unwrap(); // cannot be empty
        for channel in 0..3 {
            let histogram = &mut histogram[channel as usize];
            for x in histogram {
                *x = ((*x as f64) * HISTOGRAM_HEIGHT as f64 / (max_height as f64)) as u32
            }
        }
        histogram
    };

    let histogram_image = ImageBuffer::from_fn(256, HISTOGRAM_HEIGHT, |x, y| {
        let y = HISTOGRAM_HEIGHT - y;
        let mut pixel = [0u8; 3];
        for channel in 0..3 {
            let height = normalized_histogram[channel][x as usize];
            if y <= height {
                pixel[channel] = 255;
            }
        }
        image::Rgb(pixel)
    });

    println!("Saving histogram to {}", path.to_str().unwrap());
    histogram_image.save(path).unwrap();
}
