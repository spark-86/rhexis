use crate::flux::payload::PayloadType;

pub struct TransformPattern {
    pub alias: Option<String>,
    pub key: Option<String>,
    pub schema: Option<String>,
    pub payload_type: PayloadType,
    pub required_fields: Option<Vec<String>>,
    pub flags: Vec<String>,
}

impl TransformPattern {
    pub fn new() -> Self {
        Self {
            alias: None,
            key: None,
            schema: None,
            payload_type: PayloadType::Any,
            required_fields: None,
            flags: vec![],
        }
    }
}
