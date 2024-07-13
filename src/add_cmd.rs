use crate::cli::Add;
use crate::parser::parse_task;
use crate::todoist_client::TodoistClient;

fn notify() {
    let notifcation = libnotify::Notification::new("Rdoist", "Task added", None);
    notifcation.show().unwrap();
}

pub async fn run(client: &TodoistClient, target: &Add) {
    match &target {
        Add::Task { content } => {
            let mut task = parse_task(&content);
            task.project_id = client.get_project_id(&task.project.as_ref().unwrap()).await.clone();

            client.add_task(&task).await;
            notify();
        }
    }
}
