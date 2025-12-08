use crate::{
    flux::item::FluxItem,
    hpc::{context::Fault, directive::KernelDirective},
};

pub struct TransformReturn {
    pub ok: bool,
    pub flux_out: Vec<FluxItem>,
    pub directives: Vec<KernelDirective>,
    pub faults: Vec<Fault>,
}
