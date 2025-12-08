use crate::flux::{json::JsonFluxItem, meta::FluxMeta, payload::FluxPayload};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluxItem {
    pub name: String,
    pub schema: Option<String>,
    pub payload: FluxPayload,
    pub meta: FluxMeta,
}

impl FluxItem {
    pub fn new(name: String, schema: Option<String>, payload: FluxPayload, meta: FluxMeta) -> Self {
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

    pub fn from_cbor(cbor: &[u8]) -> Self {
        serde_cbor::from_slice(cbor).unwrap()
    }
    pub fn from_json(json: &str) -> Self {
        let parsed: JsonFluxItem = serde_json::from_str(json).unwrap();
        let payload = parsed.payload.into_real().unwrap();
        let meta = FluxMeta {
            creator: parsed.meta.creator,
            timestamp: parsed.meta.timestamp,
        };
        Self {
            name: parsed.name,
            schema: parsed.schema,
            payload,
            meta,
        }
    }
}
