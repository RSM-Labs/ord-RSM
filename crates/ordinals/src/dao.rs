use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq)]
pub struct Dao {
    pub started: Option<bool>,
    pub vote_limit: Option<u32>,
    pub time_lock: Option<u32>,
}