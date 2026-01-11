use serde::{Deserialize, Serialize};

use crate::usher::Usher;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scope {
    pub name: String,
    pub ushers: Option<Vec<Usher>>,
    pub policy: Option<String>,
    pub last_updated: u64,
}
