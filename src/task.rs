use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub content: String,
    pub project: Option<String>,
    pub project_id: Option<String>
}

impl Task {
    pub fn new(content: &String, project: Option<String>, project_id: Option<String>) -> Task {
        Task {
            content: content.to_string(),
            project,
            project_id
        }
    }
}
