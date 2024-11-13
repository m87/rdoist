use ureq::Agent;
use crate::cli::List;
use crate::config::Config;
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

pub fn run(agent: &Agent, config: &Config, target: &List) {
    match target {
        List::Project {} => {
            let projects = list_projects(agent, &config.api.token);

            match config.json {
                Some(v) => if v {
                    println!("{}", serde_json::to_string(&projects).unwrap());
                } else {
                    for project in projects {
                        project.print();
                    }
                },
                None => for project in projects {
                    project.print();
                }
            }
        }
    }
}

