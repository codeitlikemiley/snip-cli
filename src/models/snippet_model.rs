use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Snippet {
    pub prefix: String,
    pub body: Vec<String>,
    pub description: String,
}
