use crate::{
    flux::item::FluxItem,
    hpc::{context::Fault, directive::KernelDirective},
};

pub struct TransformContext<'a> {
    pub input: &'a [FluxItem],
    pub output: &'a mut Vec<FluxItem>,
    pub directives: &'a mut Vec<KernelDirective>,
    pub diag: &'a mut Vec<Fault>,
    pub hpc_calls: &'a mut Vec<(String, Vec<u8>)>,
}
