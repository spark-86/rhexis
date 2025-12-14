use crate::{
    flux::item::FluxItem,
    hpc::{context::Fault, directive::MembraneDirective},
};

pub struct HpcReturn {
    pub ok: bool,
    pub output: Option<Vec<FluxItem>>,
    pub directives: Option<Vec<MembraneDirective>>,
    pub faults: Option<Vec<Fault>>,
}
