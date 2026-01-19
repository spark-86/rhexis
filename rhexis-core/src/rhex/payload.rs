use crate::flux::payload::FluxPayload;
use crate::rhex::payload_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]

pub enum RhexPayload {
    Json(serde_json::Value),
    Binary {
        #[serde(with = "serde_bytes")]
        data: Vec<u8>,
    },
    Mixed {
        meta: serde_json::Value,
        #[serde(with = "payload_bytes")]
        data: Vec<Vec<u8>>,
    },
    None,
}

impl RhexPayload {
    pub fn as_bytes(&self) -> Vec<u8> {
        let bytes = serde_cbor::to_vec(&self);
        bytes.unwrap()
    }
}

impl Default for RhexPayload {
    fn default() -> Self {
        Self::None
    }
}

impl From<FluxPayload> for RhexPayload {
    fn from(payload: FluxPayload) -> Self {
        match payload {
            FluxPayload::Json(v) => Self::Json(v),
            FluxPayload::Binary(data) => Self::Binary { data },
            FluxPayload::Mixed { meta, data } => Self::Mixed { meta, data },
            FluxPayload::None => Self::None,
        }
    }
}
