use crate::rhex::payload::RhexPayload;
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexIntent {
    #[serde(default)]
    pub previous_hash: Binding<[u8; 32]>,
    #[serde(default)]
    pub scope: Binding<String>,
    #[serde(default = "RhexIntent::gen_nonce")]
    pub nonce: [u8; 32],
    #[serde(default)]
    pub author_public_key: Binding<[u8; 32]>,
    #[serde(default)]
    pub usher_public_key: Binding<[u8; 32]>,
    #[serde(default)]
    pub schema: Binding<String>,
    #[serde(default)]
    pub record_type: Binding<String>,
    #[serde(default)]
    pub data: RhexPayload,
}

impl RhexIntent {
    pub fn new(nonce: [u8; 32]) -> Self {
        Self {
            previous_hash: Binding::Unbound,
            scope: Binding::Unbound,
            nonce,
            author_public_key: Binding::Unbound,
            usher_public_key: Binding::Unbound,
            schema: Binding::Unbound,
            record_type: Binding::Unbound,
            data: RhexPayload::None,
        }
    }

    pub fn gen_nonce() -> [u8; 32] {
        random()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "state", content = "value")]
pub enum Binding<T> {
    Unbound,
    Bound(T),
}

impl<T> Default for Binding<T> {
    fn default() -> Self {
        Self::Unbound
    }
}
