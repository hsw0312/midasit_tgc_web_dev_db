use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    pub name: String,
    pub age: i32,
}
