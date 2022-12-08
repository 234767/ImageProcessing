use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Clone, serde::Serialize)]
pub struct PathChangeEventArgs {
    pub path: String,
}

#[must_use]
pub fn prepare_new_filename(dir_path: &Path, file_prefix: &str) -> PathBuf {
    for file in fs::read_dir(dir_path).unwrap() {
        if file
            .as_ref()
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with(file_prefix)
        {
            fs::remove_file(file.unwrap().path()).unwrap();
        }
    }

    return dir_path.join(format!("{}{}.bmp", file_prefix, Uuid::new_v4()));
}
