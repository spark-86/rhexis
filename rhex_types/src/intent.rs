#[derive(Debug)]
pub struct RhexIntent {
    pub previous_hash: Option<[u8; 32]>,
    pub scope: String,
    pub nonce: String,
    pub author_public_key: [u8; 32],
    pub usher_public_key: [u8; 32],
    pub record_type: String,
    pub data: serde_json::Value,
}

impl RhexIntent {
    pub fn new() -> Self {
        RhexIntent {
            previous_hash: None,
            scope: String::new(),
            nonce: String::new(),
            author_public_key: [0; 32],
            usher_public_key: [0; 32],
            record_type: String::new(),
            data: serde_json::Value::Null,
        }
    }
}
