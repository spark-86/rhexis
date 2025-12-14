use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MembraneDirective {
    RegisterResource {
        logical_id: Vec<u8>,
        backing: ResourceBacking,
        token: Vec<u8>,
        cause: Option<Vec<u8>>,
    },

    ReleaseResource {
        logical_id: Vec<u8>,
    },

    IoComplete {
        logical_id: Vec<u8>,
        bytes_read: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBacking {
    pub kind: String,
    pub bytes: Vec<u8>,
}

impl ResourceBacking {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.kind.as_bytes());
        bytes.extend_from_slice(&self.bytes.len().to_be_bytes());
        bytes.extend_from_slice(&self.bytes);
        bytes
    }
}
