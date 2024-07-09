use crate::project::Project;
use reqwest;
use reqwest::header::HeaderMap;

pub struct TodoistClient {
    client: reqwest::Client,
}

impl TodoistClient {
    pub fn new(token: String) -> Self {
        let mut headers = HeaderMap::new();
        let mut auth_header = String::from("Bearer ");
        auth_header.push_str(&token);
        headers.insert("Authorization", auth_header.parse().unwrap());
        TodoistClient {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub async fn get_projects(&self) -> Vec<Project> {
        let resp = self
            .client
            .get("https://api.todoist.com/rest/v2/projects")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let dupa: Vec<Project> = serde_json::from_str(&resp).unwrap();
        return dupa;
    }
}
