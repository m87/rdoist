use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    pub id: String
}

impl Project {
    pub fn print(&self) {
        println!("- {:?}", self.name);
    }
}
