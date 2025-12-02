use crate::{flux::item::FluxItem, hpc::HighPerformanceCapability};

pub mod probe;

pub trait Membrane {
    fn init(
        &self,
        hpc_binaries: Vec<Vec<u8>>,
        transform_binaries: Vec<Vec<u8>>,
    ) -> anyhow::Result<()>;

    fn initial_flux(&self) -> Vec<FluxItem>;
    fn hpc_providers(&self) -> Vec<Box<dyn HighPerformanceCapability>>;
}
