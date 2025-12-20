use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexContext {
    pub at: u64,
    pub spacial_ref: Option<String>,
    pub spacial_data: Option<Vec<u8>>,
}

impl RhexContext {
    pub fn new() -> Self {
        Self {
            at: 0,
            spacial_ref: None,
            spacial_data: None,
        }
    }
}
