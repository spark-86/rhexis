use std::collections::HashMap;

use rhexis_core::{flux::item::FluxItem, membrane::Membrane, transform::Transform};

pub mod json_path;
pub mod scoring;

pub struct Kernel {
    pub flux_pond: HashMap<String, FluxItem>,
    pub transform_registry: HashMap<String, (Box<dyn Transform>, [u8; 32])>,
    membrane: Box<dyn Membrane>,
}

impl Kernel {
    pub fn new(
        inital_flux: Vec<FluxItem>,
        inital_transforms: Vec<(Box<dyn Transform>, [u8; 32])>,
        membrane: Box<dyn Membrane>,
    ) -> Self {
        let mut pond = HashMap::new();
        for flux_item in inital_flux {
            pond.insert(flux_item.name.clone(), flux_item);
        }
        let mut registry = HashMap::new();
        for transform in inital_transforms {
            registry.insert(
                transform.0.signature().id.to_string(),
                (transform.0, transform.1),
            );
        }
        Self {
            flux_pond: pond,
            transform_registry: registry,
            membrane,
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

    pub fn resolve(&self) {
        let before = self.hash_flux();
        let mut score_map = HashMap::new();

        let mut consumed_master = Vec::new();
        let mut collapse_map = HashMap::new();

        for (transform, hash) in self.transform_registry.values() {
            let (score, observed, consumed) =
                scoring::score_transform(&transform.signature(), &self.flux_pond);
            if score == 0 {
                continue;
            }
            score_map.insert(
                transform.signature().id.to_string(),
                (score, observed.clone(), consumed.clone()),
            );
            for flux_name in consumed {
                if !consumed_master.contains(&flux_name) {
                    consumed_master.push(flux_name.to_string());
                }
            }
        }
        for transform_id in score_map.keys() {
            let (score, observed, consumed) = score_map.get(transform_id).unwrap();
            let (transform, hash) = self.transform_registry.get(transform_id).unwrap();
            let membrane = &self.membrane;
            let observed_flux: Vec<&FluxItem> = observed
                .iter()
                .map(|name| self.flux_pond.get(name).unwrap())
                .collect();
            let consumed_flux: Vec<&FluxItem> = consumed
                .iter()
                .map(|name| self.flux_pond.get(name).unwrap())
                .collect();
            let results = transform.run(observed_flux, consumed_flux, membrane);
            if results.is_ok() {
                let results = results.unwrap();
                if results.is_some() {
                    let results = results.unwrap();
                    if results.len() > 0 {
                        for flux_item in results.clone() {
                            if let Some((_, old_score)) = collapse_map.get(&flux_item.name) {
                                if score > old_score {
                                    collapse_map.insert(
                                        flux_item.name.clone(),
                                        (flux_item.clone(), *score),
                                    );
                                }
                            }
                            collapse_map.insert(flux_item.name.clone(), (flux_item, *score));
                        }
                    }
                }
            }
        }
        let after = self.hash_flux();
        if before != after {
            self.resolve();
        }
    }
}
