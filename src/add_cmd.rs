use std::fmt::write;

use ureq::Agent;

use crate::cli::{Add};
use crate::parser::{self, parse_task};
use crate::list_cmd::list_projects;

fn add_task(agent: &Agent, token: &String, content: &String) {
    let mut auth_header = String::from("Bearer ");
    auth_header.push_str(token);

    let mut task = parse_task(content);
    let projects = list_projects(agent, token);

    for project in &projects {
        if Some(project.name.clone()) == task.project {
            task.project_id = Some(project.id.clone());
            break;
        }
    }

    agent.post("https://api.todoist.com/rest/v2/tasks")
        .set("Authorization", &auth_header)
        .set("Content-Type", "application/json")
        .send_json(ureq::json!(task)).unwrap();
}


pub fn run(agent: &Agent, token: &String, target: &Add) {
    match target {
        Add::Task { content } => {
            add_task(agent, token, content);
        }
        Add::Project {} => {
            panic!();
        }
    }
}
