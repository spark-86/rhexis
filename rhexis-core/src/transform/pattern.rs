use crate::flux::payload::PayloadType;

pub struct TransformPattern {
    pub key: Option<String>,
    pub schema: Option<String>,
    pub payload_type: PayloadType,
    pub required_fields: Option<Vec<String>>,
}

impl TransformPattern {
    pub fn new() -> Self {
        Self {
            key: None,
            schema: None,
            payload_type: PayloadType::Any,
            required_fields: None,
        }
    }
}
