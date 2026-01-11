use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexAwaitingQuorum {
    pub previous_hash: [u8; 32],
    pub scope: String,
    pub count_needed: usize,
}
