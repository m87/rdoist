use ureq::Agent;
use crate::cli::List;
use crate::project::Project;

pub fn list_projects(agent: &Agent, token: &String) -> Vec<Project> {
    let mut auth_header = String::from("Bearer ");
    auth_header.push_str(token);

    agent.get("https://api.todoist.com/rest/v2/projects")
        .set("Authorization", &auth_header)
        .set("Content-Type", "application/json")
        .call().unwrap()
        .into_json().unwrap()
}

pub fn run(agent: &Agent, token: &String, target: &List) {
    match target {
        List::Project {} => {
            let projects = list_projects(agent, token);

            for project in projects {
                project.print();
            }
        }
    }
}

