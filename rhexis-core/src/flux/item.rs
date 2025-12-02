use crate::flux::{meta::FluxMeta, payload::FluxPayload};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluxItem {
    pub name: String,
    pub schema: String,
    pub payload: FluxPayload,
    pub meta: FluxMeta,
}

impl FluxItem {
    pub fn new(name: String, schema: String, payload: FluxPayload, meta: FluxMeta) -> Self {
        Self {
            name,
            schema,
            payload,
            meta,
        }
    }
    pub fn to_cbor(&self) -> Vec<u8> {
        serde_cbor::to_vec(self).unwrap()
    }
}
