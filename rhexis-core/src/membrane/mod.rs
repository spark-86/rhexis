use serde::{Deserialize, Serialize};

use crate::{flux::item::FluxItem, hpc::directive::ResourceBacking};

pub trait Membrane {
    fn execute_hpc_calls(&mut self, calls: Vec<HpcCall>) -> Vec<FluxItem>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HpcCall {
    pub name: String,
    pub logical_id: Option<Vec<u8>>,
    pub thread: String,
    pub token: Option<Vec<u8>>,
    pub input: Vec<u8>,
    pub cause: Option<Vec<u8>>,
    pub correlation: Option<[u8; 32]>,
}

impl HpcCall {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            logical_id: None,
            thread: "".to_string(),
            token: None,
            input: Vec::new(),
            cause: None,
            correlation: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembraneAction {
    pub action: ActionType,
    pub logical_id: Vec<u8>,
    pub backing: ResourceBacking,
    pub cause: Option<Vec<u8>>,
    pub correlation: Option<[u8; 32]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    RegisterResource,
    ReleaseResource,
    IoComplete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CauseHeader {
    pub target: String,
    pub thread: String,
    pub schema: String,
    pub payload: Vec<u8>,
}

impl CauseHeader {
    pub fn new() -> Self {
        Self {
            target: "".to_string(),
            thread: "".to_string(),
            schema: "".to_string(),
            payload: vec![],
        }
    }
}
