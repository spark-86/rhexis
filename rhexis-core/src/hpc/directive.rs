use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MembraneDirective {
    RegisterResource {
        logical_id: Vec<u8>,
        backing: ResourceBacking,
        token: Vec<u8>,
        cause: Option<Vec<u8>>,
        correlation: Option<[u8; 32]>,
    },

    ReleaseResource {
        logical_id: Vec<u8>,
        correlation: Option<[u8; 32]>,
    },

    IoComplete {
        logical_id: Vec<u8>,
        bytes: usize,
        correlation: Option<[u8; 32]>,
        cause: Vec<u8>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBacking {
    pub kind: String,
    pub bytes: Option<Vec<u8>>,
}

impl ResourceBacking {
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_cbor::to_vec(self).unwrap()
    }
}
