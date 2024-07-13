use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    content: String,
    project_id: String
}

impl Task {
    pub fn new(content: &String, project: &String) -> Task {
        Task {
            content: content.to_string(),
            project_id: project.to_string()
        }
    }
}
