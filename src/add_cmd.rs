use crate::cli::Add;
use crate::todoist_client::TodoistClient;

pub async fn run(client: &TodoistClient, target: &Add) {
    match &target {
        Add::Task { name } => {
            let projects = client.get_projects().await;

            for project in projects {
                project.print();
            }
        }
    }
}
