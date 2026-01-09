use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WorkLog {
    pub id: u32,
    pub project: String,
    pub hours: f32,
    pub date: String,
}
