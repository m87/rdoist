use crate::cli::Add;
use crate::todoist_client::TodoistClient;

fn notify() {
    let notifcation = libnotify::Notification::new("Rdoist", "Task added", None);
    notifcation.show().unwrap();
}

pub async fn run(client: &TodoistClient, target: &Add) {
    match &target {
        Add::Task { content } => {
            client.add_task(content, &String::from("")).await;
            notify();
        }
    }
}
