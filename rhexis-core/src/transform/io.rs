use serde::{Deserialize, Serialize};

use crate::{flux::item::FluxItem, hpc::directive::KernelDirective};

#[derive(Debug, Clone, Deserialize)]
pub struct TransformInput {
    pub flux_in: Vec<FluxItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransformOutput {
    pub ok: bool,
    pub flux_out: Vec<FluxItem>,
    pub directives: Vec<KernelDirective>,
    pub faults: Vec<Fault>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fault {
    pub kind: String,
    pub message: String,
}
