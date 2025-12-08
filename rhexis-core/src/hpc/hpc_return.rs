use crate::{
    flux::item::FluxItem,
    hpc::{context::Fault, directive::KernelDirective},
};

pub struct HpcReturn {
    pub ok: bool,
    pub output: Option<Vec<FluxItem>>,
    pub directives: Option<Vec<KernelDirective>>,
    pub faults: Option<Vec<Fault>>,
}
