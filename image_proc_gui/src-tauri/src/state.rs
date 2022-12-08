use image::RgbImage;
use image_proc::modifications::Transformation;
use lazy_static::lazy_static;
use std::default::Default;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Default)]
pub struct AppState {
    pub active_image: Option<RgbImage>,
    pub original_image: Option<RgbImage>,
    pub reference_image: Option<RgbImage>,
    pub temp_dir: PathBuf,
}

impl AppState {
    pub fn open_image(&mut self, path: &String) -> Result<(), String> {
        let image = match image::io::Reader::open(path) {
            Ok(reader) => match reader.decode() {
                Ok(img) => Some(img.to_rgb8()),
                _ => None,
            },
            _ => None,
        };
        self.active_image = image.clone();
        self.original_image = image;
        match &self.active_image {
            Some(_) => Ok(()),
            None => Err(format!("Failed to open file {}", path)),
        }
    }

    pub fn apply_transform(&mut self, transform: &impl Transformation) {
        match &mut self.active_image {
            Some(img) => transform.apply(img),
            None => return,
        }
    }
}

lazy_static! {
    pub static ref STATE: Mutex<AppState> = Mutex::new(AppState {
        active_image: None,
        original_image: None,
        reference_image: None,
        temp_dir: {
            let path = std::env::temp_dir().join("image-proc");
            if !path.exists() {
                std::fs::create_dir(&path).unwrap();
            }
            path
        }
    });
}
