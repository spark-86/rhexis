use serde::{Deserialize, Serialize};

use crate::rhex::{context::RhexContext, intent::RhexIntent, signature::RhexSignature};

pub mod context;
pub mod intent;
pub mod payload;
pub mod signature;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rhex {
    pub magic: [u8; 6],
    pub intent: RhexIntent,
    pub context: RhexContext,
    pub signature: RhexSignature,
    pub current_hash: Option<[u8; 32]>,
}

impl Rhex {
    pub fn new() -> Self {
        Self {
            magic: *b"RHEX01",
            intent: RhexIntent::new(RhexIntent::gen_nonce()),
            context: RhexContext::new(),
            signature: RhexSignature::new(),
            current_hash: None,
        }
    }
}

impl Default for Rhex {
    fn default() -> Self {
        Self::new()
    }
}
