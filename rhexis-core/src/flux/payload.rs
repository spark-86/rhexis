use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FluxPayload {
    Json(Value),
    Binary(Vec<u8>),
    Mixed { meta: Value, data: Vec<Vec<u8>> },
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayloadType {
    Json,
    Binary,
    Mixed,
    None,
    Any,
}

impl FluxPayload {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            FluxPayload::Json(v) => serde_json::to_vec(v).unwrap(),
            FluxPayload::Binary(v) => v.clone(),
            FluxPayload::Mixed { meta, data } => {
                let _ = meta;
                data.iter().flatten().cloned().collect()
            }
            FluxPayload::None => vec![],
        }
    }
}
