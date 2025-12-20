use serde::{Deserialize, Serialize};

use crate::flux::payload::PayloadType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HpcDescriptor {
    pub descriptor_ver: u32,
    pub name: String,
    pub capability: String,
    pub version: String,
    pub requires: Vec<String>,
    pub bin_format: BinaryFormat,
    pub blake3: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformDescriptor {
    pub descriptor_ver: u32,
    pub name: String,
    pub version: String,
    pub requires: Vec<String>,
    pub views: Vec<String>,
    pub interacts: Vec<PatternDescriptor>,
    pub bind: Option<String>,
    pub effects: Vec<PatternDescriptor>,
    pub bin_format: BinaryFormat,
    pub blake3: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDescriptor {
    pub key: Option<String>,
    pub thread: String,
    pub schema: Option<String>,
    pub payload_type: PayloadType,
    pub required_fields: Option<Vec<String>>,
    pub flags: Vec<String>,
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
