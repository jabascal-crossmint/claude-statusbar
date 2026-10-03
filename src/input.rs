use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub workspace: Option<Workspace>,
    pub model: Option<Model>,
    pub transcript_path: Option<String>,
    pub context_window: Option<ContextWindow>,
}

#[derive(Debug, Deserialize)]
pub struct ContextWindow {
    pub used_percentage: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub current_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub display_name: Option<String>,
}
