use std::{collections::HashMap, os::fd::RawFd, sync::Arc};

use rhexis_core::{flux::item::FluxItem, registry};
use rhexis_kernel::Kernel;

pub mod loader;

pub struct Membrane {
    pub registry: registry::MembraneRegistry,
    pub socket_table: HashMap<String, RawFd>,
}

impl Membrane {
    pub fn new(
        transforms: Vec<Arc<registry::LoadedTransform>>,
        hpcs: Vec<registry::LoadedHpc>,
    ) -> Self {
        let hpc_map: HashMap<String, registry::LoadedHpc> = hpcs
            .into_iter()
            .map(|h| (h.descriptor.capability.to_string(), h))
            .collect();

        Self {
            registry: registry::MembraneRegistry {
                transforms, // ✔ Already Arc<LoadedTransform>
                hpcs: hpc_map,
            },
            socket_table: HashMap::new(),
        }
    }

    pub fn register_hpc(&mut self, hpc: registry::LoadedHpc) {
        self.registry
            .hpcs
            .insert(hpc.descriptor.capability.to_string(), hpc);
    }

    pub fn register_transform(&mut self, transform: Arc<registry::LoadedTransform>) {
        self.registry.transforms.push(transform);
    }

    pub fn unload_hpc(&mut self, capability: &str) {
        self.registry.hpcs.remove(capability);
    }

    pub fn has_requirement(&self, capability: &str) -> bool {
        self.registry.hpcs.contains_key(capability)
    }

    pub fn spin_kernel(&self, flux: &[FluxItem]) -> anyhow::Result<()> {
        let hpc_symbols = self.registry.hpcs.keys().cloned().collect::<Vec<String>>();

        // ✔ The kernel wants a slice of Arc<LoadedTransform>
        let transform_slice = self.registry.transforms.as_slice();

        let kernel = Kernel::new(
            flux.to_vec(),
            hpc_symbols,
            transform_slice, // ✔ No clones, no Arc<vec>, just a slice
        );

        kernel.resolve(); // Or however you start ticking the universe

        Ok(())
    }
}
