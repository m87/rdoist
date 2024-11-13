use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub id: String,
}

impl Project {
    pub fn print(&self) {
        println!("- {:?}", self.name);
    }
}
