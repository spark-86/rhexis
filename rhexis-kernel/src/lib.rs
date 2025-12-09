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

    pub fn resolve(&mut self) -> Vec<HpcCall> {
        let mut score_map = HashMap::new();

        let mut consumed_master = Vec::new();
        let mut collapse_map = HashMap::new();
        let mut diag_master = Vec::new();
        let mut directives_master = Vec::new();
        let mut hpc_calls_master = Vec::new();

        for transform in self.transform_registry.values() {
            let (score, observed, consumed) =
                scoring::score_transform(&transform.descriptor, &self.flux_pond);
            if score == 0 {
                continue;
            }
            score_map.insert(
                transform.descriptor.name.to_string(),
                (score, observed.clone(), consumed.clone()),
            );
            for flux_name in consumed {
                if !consumed_master.contains(&flux_name) {
                    consumed_master.push(flux_name.to_string());
                }
            }
            println!("{}: {}", transform.descriptor.name, score);
        }
        for transform_id in score_map.keys() {
            let (score, observed, consumed) = score_map.get(transform_id).unwrap();
            let transform = self.transform_registry.get(transform_id).unwrap();
            let observed_flux: Vec<&FluxItem> = observed
                .iter()
                .map(|name| self.flux_pond.get(name).unwrap())
                .collect();
            let consumed_flux: Vec<&FluxItem> = consumed
                .iter()
                .map(|name| self.flux_pond.get(name).unwrap())
                .collect();
            let mut total_flux = Vec::new();
            for flux_item in observed_flux.iter().chain(consumed_flux.iter()) {
                total_flux.push((*flux_item).clone());
            }
            let mut out_vec = Vec::new();
            let mut diag = Vec::new();
            let mut directives = Vec::new();
            let mut hpc_calls = Vec::new();
            let mut ctx = TransformContext {
                input: &total_flux,
                output: &mut out_vec,
                diag: &mut diag,
                directives: &mut directives,
                hpc_calls: &mut hpc_calls,
            };
            println!("Before transform {} entry...", transform.descriptor.name);
            let results = (transform.entry.entry)(&mut ctx);
            println!("After transform entry. Result: {}", results);
            if results == 0 {
                for flux_item in out_vec {
                    if let Some((_, old_score)) = collapse_map.get(&flux_item.name) {
                        if score > old_score {
                            collapse_map
                                .insert(flux_item.name.clone(), (flux_item.clone(), *score));
                        }
                    } else {
                        collapse_map.insert(flux_item.name.clone(), (flux_item, 100));
                    }
                }
                diag_master.append(&mut diag);
                directives_master.append(&mut directives);
                for (name, input) in hpc_calls {
                    hpc_calls_master.push(HpcCall { name, input });
                }
            }
        }
        println!("Collapse Map: {:?}", collapse_map);

        for item in consumed_master {
            self.remove_flux(&item);
        }

        for item in collapse_map.iter() {
            let flux_item = self.flux_pond.get(&item.1.0.name);
            if flux_item.is_none() {
                self.add_flux(item.1.0.clone());
            } else {
                let working = self.flux_pond.get_mut(&item.1.0.name).unwrap();
                *working = item.1.0.clone();
            }
        }

        hpc_calls_master
    }
}
