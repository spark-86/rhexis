use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluxMeta {
    pub creator: String,
    pub timestamp: u64,
}
