use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}
