use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexCommited {
    pub current_hash: [u8; 32],
    pub scope: String,
    pub previous_hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexDone {
    pub current_hash: [u8; 32],
    pub timestamp: u64,
    pub scope: String,
}
