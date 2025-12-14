use std::{collections::HashMap, sync::Arc};

use rhexis_core::{
    flux::item::FluxItem, membrane::HpcCall, registry::LoadedTransform,
    transform::context::TransformContext,
};

pub mod json_path;
pub mod scoring;

pub struct Kernel {
    pub flux_pond: HashMap<String, FluxItem>,
    pub hpc_symbols: Vec<String>,
    pub transform_registry: HashMap<String, Arc<LoadedTransform>>,
}

impl Kernel {
    pub fn new(
        inital_flux: Vec<FluxItem>,
        hpc_symbols: Vec<String>,
        inital_transforms: &[Arc<LoadedTransform>],
    ) -> Self {
        let mut pond = HashMap::new();
        for flux_item in inital_flux {
            pond.insert(flux_item.name.clone(), flux_item);
        }
        let mut registry = HashMap::new();
        for transform in inital_transforms {
            registry.insert(transform.descriptor.name.to_string(), transform.clone());
        }
        Self {
            flux_pond: pond,
            hpc_symbols,
            transform_registry: registry,
        }
    }

    pub fn add_flux(&mut self, flux_item: FluxItem) {
        self.flux_pond.insert(flux_item.name.clone(), flux_item);
    }
    pub fn get_flux(&self, name: &str) -> Option<&FluxItem> {
        self.flux_pond.get(name)
    }
    pub fn remove_flux(&mut self, name: &str) {
        self.flux_pond.remove(name);
    }

    pub fn hash_flux(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        let mut keys: Vec<&String> = self.flux_pond.keys().collect();
        keys.sort();
        for key in keys {
            let item = self.flux_pond.get(key).unwrap();
            hasher.update(&item.to_cbor());
        }
        hasher.finalize().into()
    }

    pub fn resolve(&mut self, system_flux: Vec<FluxItem>) -> Vec<HpcCall> {
        // ---- process incoming flux from the membrane -------------------------
        for flux in system_flux {
            println!("Adding {} from the membrane...", &flux.name);
            self.add_flux(flux.clone());
        }

        println!("Starting Flux: {:?}", &self.flux_pond);
        let mut score_map = HashMap::new();

        let mut consumed_master: Vec<String> = Vec::new();
        let mut collapse_map = HashMap::new();
        let mut diag_master = Vec::new();
        let mut hpc_calls_master = Vec::new();

        // ---- scoring pass ----------------------------------------------------
        for transform in self.transform_registry.values() {
            let (score, matched, consumed) =
                scoring::score_transform(&transform.descriptor, &self.flux_pond);

            if score == 0 {
                continue;
            }

            score_map.insert(
                transform.descriptor.name.clone(),
                (score, matched.clone(), consumed.clone()),
            );

            for flux_name in consumed {
                if !consumed_master.contains(&flux_name) {
                    consumed_master.push(flux_name);
                }
            }

            println!("{}: {}", transform.descriptor.name, score);
        }

        // ---- execution pass --------------------------------------------------
        for (transform_id, (score, matched, _)) in score_map.iter() {
            let transform = self.transform_registry.get(transform_id).unwrap();

            // Preserve kernel-determined order
            let total_flux: Vec<FluxItem> = matched
                .iter()
                .map(|name| self.flux_pond.get(name).unwrap().clone())
                .collect();

            let mut out_blob: Option<Vec<u8>> = None;
            let mut diag_blob: Option<Vec<u8>> = None;
            let mut hpc_calls_blob: Option<Vec<u8>> = None;

            let mut ctx = TransformContext {
                input: &serde_cbor::to_vec(&total_flux).unwrap(),
                output: &mut out_blob,
                diag: &mut diag_blob,
                hpc_calls: &mut hpc_calls_blob,
            };

            println!("Before transform {} entry...", transform.descriptor.name);
            let result = (transform.entry.entry)(&mut ctx);
            println!("After transform entry. Result: {}", result);
            let out_bin = ctx.output.clone();
            if out_bin.is_some() {
                let out_flux: Vec<FluxItem> = serde_cbor::from_slice(&out_bin.unwrap()).unwrap();
                if result == 0 {
                    for flux_item in out_flux {
                        match collapse_map.get(&flux_item.name) {
                            Some((_, old_score)) if score > old_score => {
                                collapse_map
                                    .insert(flux_item.name.clone(), (flux_item.clone(), *score));
                            }
                            None => {
                                collapse_map
                                    .insert(flux_item.name.clone(), (flux_item.clone(), *score));
                            }
                            _ => {}
                        }
                    }
                    if ctx.diag.is_some() {
                        diag_master.append(&mut ctx.diag.as_mut().unwrap().to_owned());
                    }
                    if ctx.hpc_calls.is_some() {
                        let mut hpc_call_exploded: Vec<HpcCall> =
                            serde_cbor::from_slice(&ctx.hpc_calls.as_ref().unwrap()).unwrap();
                        hpc_calls_master.append(&mut hpc_call_exploded);
                    }
                }
            }
        }

        println!("Collapse Map: {:?}", collapse_map);

        // ---- consume ---------------------------------------------------------
        for name in consumed_master {
            println!("Consumed {}", &name);
            self.remove_flux(&name);
        }

        // ---- materialize outputs ---------------------------------------------
        for (_, (flux_item, _)) in collapse_map.iter() {
            match self.flux_pond.get_mut(&flux_item.name) {
                Some(existing) => *existing = flux_item.clone(),
                None => self.add_flux(flux_item.clone()),
            }
        }
        hpc_calls_master
    }
}
