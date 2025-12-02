use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RhpKind {
    Transform,
    Hpc,
}
