use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    project_id: String,
    content: String
}
