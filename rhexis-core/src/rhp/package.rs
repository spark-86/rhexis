use serde::{Deserialize, Serialize};

use crate::rhp::{descriptor::RhpDescriptor, kind::RhpKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhpPackage {
    pub kind: RhpKind,
    pub descriptor: RhpDescriptor,
    pub binary: Vec<u8>,
}
