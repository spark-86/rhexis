use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FluxPayload {
    Json(Value),
    Binary(Vec<u8>),
    Mixed { meta: Value, data: Vec<u8> },
    None,
}

pub enum PayloadType {
    Json,
    Binary,
    Mixed,
    None,
    Any,
}
