use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Project {
    name: String,
}

impl Project {
    pub fn print(&self) {
        println!("- {:?}", self.name);
    }
}
