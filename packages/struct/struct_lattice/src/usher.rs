use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usher {
    pub name: String,
    pub public_key: [u8; 32],
    pub priority: u8,
    pub ip_address: Option<String>,
    pub port: Option<u16>,
    pub last_updated: u64,
}
