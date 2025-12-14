use crate::{flux::item::FluxItem, hpc::context::Fault};

pub struct TransformReturn {
    pub ok: bool,
    pub flux_out: Vec<FluxItem>,
    pub faults: Vec<Fault>,
}
