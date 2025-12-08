use serde::{Deserialize, Serialize};

use crate::{flux::item::FluxItem, hpc::directive::KernelDirective};

pub struct HpcContext<'a> {
    pub input: &'a [u8],
    pub output: &'a mut Vec<FluxItem>,
    pub directives: &'a mut Vec<KernelDirective>,
    pub diag: &'a mut Vec<Fault>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fault {
    pub kind: String,
    pub message: String,
}
