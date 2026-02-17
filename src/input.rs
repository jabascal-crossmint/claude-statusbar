use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub workspace: Option<Workspace>,
    pub model: Option<Model>,
    pub session_id: Option<String>,
    pub transcript_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub current_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub display_name: Option<String>,
}
