use rhexis_core::flux::item::FluxItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetFlux {
    #[serde(with = "serde_bytes")]
    pub sig: Option<[u8; 64]>,
    pub key: Option<[u8; 32]>,
    pub ip_addr: String,
    pub port: u16,
    pub payload: Vec<FluxItem>,
    pub gt: u64,
}

impl NetFlux {
    pub fn new(
        sig: Option<[u8; 64]>,
        key: Option<[u8; 32]>,
        ip_addr: String,
        port: u16,
        payload: Vec<FluxItem>,
        gt: u64,
    ) -> Self {
        Self {
            sig,
            key,
            ip_addr,
            port,
            payload,
            gt,
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.ip_addr.as_bytes());
        hasher.update(&self.port.to_be_bytes());
        let payload_bytes = serde_cbor::to_vec(&self.payload).unwrap();
        hasher.update(&payload_bytes);
        hasher.update(&self.gt.to_be_bytes());
        hasher.finalize().into()
    }

    pub fn serialize(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self).unwrap()
    }
}
