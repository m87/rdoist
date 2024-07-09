use crate::cli::List;
use crate::todoist_client::TodoistClient;

pub async fn run(client: &TodoistClient, target: &List) {
    match &target {
        List::Project {} => {
            let projects = client.get_projects().await;

            for project in projects {
                project.print();
            }
        }
    }
}
