use crate::util;
use crate::state::AppState;
use image::{ImageBuffer, RgbImage};
use image_proc::histogram::Histogram;
use std::path::Path;
use tauri::{Manager, Window, Wry};
use crate::util::prepare_new_filename;

pub fn update_active_image_histogram(state: &AppState, window: &Window<Wry>) {
    const FILE_PREFIX: &str = "histogram-original";
    match &state.active_image {
        None => {}
        Some(img) => {
            let path = prepare_new_filename(&state.temp_dir, FILE_PREFIX);
            create_histogram(img, path.as_path());
            window
                .emit_all(
                    "active-histogram-update",
                    util::PathChangeEventArgs {
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

    histogram_image.save(path).unwrap();
}
