use serde::{Deserialize, Serialize};

use crate::hpc::directive::ResourceBacking;

#[derive(Debug, Serialize, Deserialize)]
pub struct HpcCallEnvelope {
    pub logical_id: Option<Vec<u8>>,
    pub token: Option<Vec<u8>>,
    pub cause: Option<Vec<u8>>,
    pub backing: Option<ResourceBacking>,
    pub payload: Vec<u8>,
}
