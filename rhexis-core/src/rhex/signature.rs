use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexSignature {
    pub sig_type: SignatureType,
    pub public_key: [u8; 32],
    #[serde(with = "BigArray")]
    pub signature: [u8; 64],
}

impl RhexSignature {
    pub fn new() -> Self {
        Self {
            sig_type: SignatureType::None,
            public_key: [0; 32],
            signature: [0; 64],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignatureType {
    Author,
    Usher,
    Quorum,
    Observer,
    None,
}
