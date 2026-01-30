use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usher {
    pub name: String,
    #[serde(with = "serde_bytes")]
    pub public_key: [u8; 32],
    pub priority: u8,
    pub location: UsherLocation,
    pub last_updated: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UsherLocation {
    Local,
    Remote { ip_addr: String, port: u16 },
}

impl Usher {
    pub fn new(name: &str, public_key: [u8; 32], priority: u64, location: UsherLocation) -> Self {
        Self {
            name: name.to_string(),
            public_key,
            priority: priority as u8,
            location,
            last_updated: 0,
        }
    }
}
