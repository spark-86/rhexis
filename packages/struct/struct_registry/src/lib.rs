use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type Registry = HashMap<String, RegistryEntry>;

#[derive(Debug, Serialize, Deserialize)]
pub enum RegistryEntry {
    LogicalId([u8; 32]),
    String(String),
    U64(u64),
}
