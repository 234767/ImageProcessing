#[derive(Clone, serde::Serialize)]
pub struct PathChangeEventArgs {
    pub path: String,
}