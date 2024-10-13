use regex::{bytes::Regex};

use crate::task::Task;


pub fn parse_task(task: &String) -> Task {
    let project_re = Regex::new(r"#(\w+)").unwrap();

    let caps = project_re.captures(task.as_bytes());
    let project = match caps {
        Some(c) => Some(String::from_utf8(c.get(1).unwrap().as_bytes().to_vec()).unwrap()),
        None => Some(String::from("inbox"))
    };


    let content = String::from_utf8(project_re.replace_all(task.as_bytes(), b"").into_owned()).unwrap();
    Task::new(&content, project, None)
}


#[cfg(test)]
mod tests {
    use super::parse_task;


    #[test]
    fn should_get_project_name() {
        let project_name = parse_task(&"asdkjaskjd #test dalsds jsdkadsd alj".to_string()).project;

        assert_eq!(project_name.unwrap(), "test");
    }

    #[test]
    fn should_get_default_project_name() {
        let project_name = parse_task(&"asdkjaskjd dalsds jsdkadsd alj".to_string()).project;

        assert_eq!(project_name, None);
    }
}

