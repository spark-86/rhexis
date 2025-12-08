use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KernelDirective {
    CreateResource {
        kind: ResourceKind,
        id: u64,
        fd: i32,
    },
    DestroyResource {
        id: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceKind {
    Socket,
}
