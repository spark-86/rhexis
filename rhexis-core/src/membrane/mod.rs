use crate::flux::item::FluxItem;

pub trait Membrane {
    fn execute_hpc_calls(&mut self, calls: Vec<HpcCall>) -> Vec<FluxItem>;
}

pub struct HpcCall {
    pub name: String,
    pub input: Vec<u8>,
}
