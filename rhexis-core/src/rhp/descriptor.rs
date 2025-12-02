use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HpcDescriptor {
    pub name: String,
    pub capability: String,
    pub version: String,
    pub requires: Vec<String>,
    pub bin_format: BinaryFormat,
    pub blake3: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformDescriptor {
    pub name: String,
    pub version: String,
    pub requires: Vec<String>,
    pub observes: Vec<PatternDescriptor>,
    pub consumes: Vec<PatternDescriptor>,
    pub emits: Vec<PatternDescriptor>,
    pub proposes: Vec<PatternDescriptor>,
    pub bin_format: BinaryFormat,
    pub blake3: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDescriptor {
    pub key: Option<String>,
    pub schema: Option<String>,
    pub payload_type: String,
    pub required_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RhpDescriptor {
    Hpc(HpcDescriptor),
    Transform(TransformDescriptor),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryFormat {
    Native,
    Wasm,
}
