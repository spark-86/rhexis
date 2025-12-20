use crate::{
    flux::{availability::FluxAvailability, meta::FluxMeta},
    rhex::intent::RhexIntent,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluxItem {
    pub name: String,
    pub thread: String,
    pub availability: FluxAvailability,
    pub intent: RhexIntent,
    pub correlation: Option<[u8; 32]>,
    pub meta: FluxMeta,
}

impl FluxItem {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            thread: "".to_string(),
            availability: FluxAvailability::Eventually,
            intent: RhexIntent::new(RhexIntent::gen_nonce()),
            correlation: None,
            meta: FluxMeta {
                creator: "".to_string(),
                timestamp: 0,
            },
        }
    }
    pub fn to_cbor(&self) -> Vec<u8> {
        serde_cbor::to_vec(self).unwrap()
    }

    pub fn from_cbor(cbor: &[u8]) -> Self {
        serde_cbor::from_slice(cbor).unwrap()
    }
}
