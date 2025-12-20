use std::{collections::HashMap, sync::Arc};

use rhexis_core::{
    flux::item::FluxItem,
    hpc::{
        context::HpcContext,
        directive::{MembraneDirective, ResourceBacking},
        envelope::HpcCallEnvelope,
    },
    membrane::{HpcCall, Membrane},
    registry,
};
use rhexis_kernel::Kernel;

pub mod directives;
pub mod loader;

pub struct MacOSMembrane {
    pub registry: registry::MembraneRegistry,
    pub resource_table: HashMap<Vec<u8>, (Vec<u8>, ResourceBacking)>,
}

impl Membrane for MacOSMembrane {
    fn execute_hpc_calls(&mut self, calls: Vec<HpcCall>) -> Vec<FluxItem> {
        let mut call_output: Vec<FluxItem> = Vec::new();
        //let mut membrane_dir_blobs: Vec<Vec<u8>> = Vec::new();
        let mut all_directives: Vec<MembraneDirective> = Vec::new();

        for call in calls {
            let hpc = self.registry.hpcs.get(&call.name).unwrap();
            let mut token = None;

            // ---- gather backing/token if resource already exists ----
            let mut backing = None;
            if let Some(ref logical_id) = call.logical_id {
                if let Some((tok, back)) = self.resource_table.get(logical_id) {
                    token = Some(tok.clone());
                    backing = Some(back.clone());
                }
            }

            let corr = call.correlation;

            // ---- build envelope ----
            let envelope = HpcCallEnvelope {
                logical_id: call.logical_id,
                thread: call.thread.clone(),
                token,
                cause: call.cause,
                backing,
                payload: call.input,
            };

            let input_bytes = serde_cbor::to_vec(&envelope).unwrap();

            // ---- per-call outputs (CBOR blobs) ----
            let mut flux_blob: Option<Vec<u8>> = None;
            let mut directives_blob: Option<Vec<u8>> = None;
            let mut diag = Vec::new();

            let mut ctx = HpcContext {
                input: input_bytes.as_slice(),
                output: &mut flux_blob,
                directives: &mut directives_blob,
                diag: &mut diag,
            };
            println!("Calling HPC capability: {}", &call.name);
            unsafe {
                (hpc.entry.entry)(&mut ctx);
            }
            // ---- decode flux output ----
            if let Some(blob) = flux_blob {
                let mut flux_items: Vec<FluxItem> = serde_cbor::from_slice(&blob).unwrap();

                for flux in flux_items.iter_mut() {
                    flux.thread = call.thread.clone();
                    if flux.correlation.is_some() {
                        flux.correlation = corr.clone();
                    }
                }

                call_output.extend(flux_items);
            }

            // ---- collect directive blobs ----
            if let Some(blob) = directives_blob {
                let membrane_dirs: Vec<MembraneDirective> = serde_cbor::from_slice(&blob).unwrap();
                for dir in membrane_dirs {
                    let updated_dir = match dir {
                        MembraneDirective::RegisterResource {
                            logical_id,
                            backing,
                            token,
                            cause,
                            correlation: _,
                        } => MembraneDirective::RegisterResource {
                            logical_id,
                            backing,
                            token,
                            cause,
                            correlation: corr.clone(),
                        },
                        MembraneDirective::ReleaseResource {
                            logical_id,
                            correlation: _,
                        } => MembraneDirective::ReleaseResource {
                            logical_id,
                            correlation: corr.clone(),
                        },
                        MembraneDirective::IoComplete {
                            logical_id,
                            bytes,
                            correlation: _,
                            cause,
                        } => MembraneDirective::IoComplete {
                            logical_id,
                            bytes,
                            correlation: corr.clone(),
                            cause,
                        },
                    };
                    all_directives.push(updated_dir.clone());
                    //membrane_dir_blobs.push(serde_cbor::to_vec(&updated_dir).unwrap());
                }
            }
        }

        // ---- process directives into membrane flux ----
        if let Some(membrane_flux) =
            directives::process_directives(&all_directives, &mut self.resource_table).unwrap()
        {
            for flux in membrane_flux {
                call_output.push(flux);
            }
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
            resource_table: HashMap::new(),
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

        let mut system_flux = Vec::new();
        let mut done = false;
        while !done {
            println!("---------------------------------------");
            println!("Start of kernel spin...");
            let before = kernel.hash_flux();
            println!("Flux hash at start: {:?}", &before);
            let hpc_calls = kernel.resolve(system_flux);

            println!("End of kernel spin. Firing HPCs...");

            let hpc_returns = self.execute_hpc_calls(hpc_calls);

            println!("Returned: {:?}", hpc_returns);
            // Poll shit in here somewhere.
            system_flux = hpc_returns.clone();
            let after = kernel.hash_flux();
            println!("Flux hash at end: {:?}", &after);
            if hpc_returns.len() == 0 {
                if before == after {
                    done = true;
                }
            }
        }
        Ok(())
    }
}
