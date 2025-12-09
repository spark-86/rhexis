use std::{collections::HashMap, os::fd::RawFd, sync::Arc};

use rhexis_core::{
    flux::item::FluxItem,
    hpc::context::HpcContext,
    membrane::{HpcCall, Membrane},
    registry,
};
use rhexis_kernel::Kernel;

pub mod loader;

pub struct MacOSMembrane {
    pub registry: registry::MembraneRegistry,
    pub socket_table: HashMap<String, RawFd>,
}

impl Membrane for MacOSMembrane {
    fn execute_hpc_calls(&mut self, calls: Vec<HpcCall>) -> Vec<FluxItem> {
        let mut call_output = Vec::new();

        for call in calls {
            let hpc = self.registry.hpcs.get(&call.name).unwrap();
            let mut output = Vec::new();
            let mut directives = Vec::new();
            let mut diag = Vec::new();

            let mut ctx = HpcContext {
                input: &call.input,
                output: &mut output,
                directives: &mut directives,
                diag: &mut diag,
            };
            unsafe {
                (hpc.entry.entry)(&mut ctx);
            }
            call_output.append(&mut output);
        }
        call_output
    }
}

impl MacOSMembrane {
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

    pub fn spin_kernel(&mut self, flux: &[FluxItem]) -> anyhow::Result<()> {
        let hpc_symbols = self.registry.hpcs.keys().cloned().collect::<Vec<String>>();

        // ✔ The kernel wants a slice of Arc<LoadedTransform>
        let transform_slice = self.registry.transforms.as_slice();

        println!("Init kernel...");
        let mut kernel = Kernel::new(
            flux.to_vec(),
            hpc_symbols,
            transform_slice, // ✔ No clones, no Arc<vec>, just a slice
        );
        let mut done = false;
        while !done {
            println!("Start of kernel spin...");
            let before = kernel.hash_flux();

            let hpc_calls = kernel.resolve(); // Or however you start ticking the universe

            println!("End of kernel spin. Firing HPCs...");

            let hpc_returns = self.execute_hpc_calls(hpc_calls);

            println!("\nReturned: {:?}", hpc_returns);
            // Poll shit in here somewhere.

            let after = kernel.hash_flux();

            if hpc_returns.len() == 0 {
                if before == after {
                    done = true;
                }
            }
        }
        Ok(())
    }
}
