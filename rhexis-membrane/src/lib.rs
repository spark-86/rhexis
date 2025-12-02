use std::collections::HashMap;

pub mod loader;
pub mod registry;

pub struct Membrane {
    pub registry: registry::MembraneRegistry,
}

impl Membrane {
    pub fn new() -> Self {
        Self {
            registry: registry::MembraneRegistry {
                transforms: HashMap::new(),
                hpcs: HashMap::new(),
            },
        }
    }

    pub fn register_hpc(&mut self, hpc: registry::LoadedHpc) {
        self.registry
            .hpcs
            .insert(hpc.descriptor.capability.to_string(), hpc);
    }

    pub fn has_requirement(&self, capability: &str) -> bool {
        self.registry.hpcs.contains_key(capability)
    }
}
