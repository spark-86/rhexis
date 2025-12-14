use serde::{Deserialize, Serialize};

pub struct HpcContext<'a> {
    pub input: &'a [u8],
    pub output: &'a mut Option<Vec<u8>>,
    pub directives: &'a mut Option<Vec<u8>>,
    pub diag: &'a mut Vec<Fault>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fault {
    pub kind: String,
    pub message: String,
}
