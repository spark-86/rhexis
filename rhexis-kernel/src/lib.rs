use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use rhexis_core::{
    flux::item::FluxItem, membrane::HpcCall, registry::LoadedTransform, rhex::intent::Binding,
};

pub mod cleanup;
pub mod dump_flux;
pub mod execute_single;
pub mod execute_transforms;
pub mod handle_corr;
pub mod handle_outputs;
pub mod ingest_flux;
pub mod json_path;
pub mod resolve;
pub mod scoring;
pub mod trans_output;
pub mod update_pressure;

pub struct ScoreResult {
    score: usize,
    matched: Vec<String>,
    consumed: Vec<String>,
    bound: Option<String>,
}

pub struct ExecutionArtifacts {
    collapse_map: HashMap<String, (FluxItem, usize)>,
    consumed: Vec<String>,
    detonators: Vec<[u8; 32]>,
    hpc_calls: Vec<HpcCall>,
    diag: Vec<u8>,
}

type ThreadId = String;
type Schema = String;
type FluxPond = HashMap<ThreadId, HashMap<Schema, Vec<FluxItem>>>;

pub struct Kernel {
    pub flux_pond: FluxPond,
    pub thread_pressure: HashMap<ThreadId, usize>,
    pub overflow: HashMap<ThreadId, Vec<FluxItem>>,
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
            let status = Kernel::add_flux(&mut pond, flux_item.clone());
            match status {
                Err(e) => println!("Borked ({:?}) loading flux: {:?}", e, &flux_item.clone()),
                _ => {}
            }
        }
        let mut registry = HashMap::new();
        for transform in inital_transforms {
            registry.insert(transform.descriptor.name.to_string(), transform.clone());
        }
        Self {
            flux_pond: pond,
            thread_pressure: HashMap::new(),
            overflow: HashMap::new(),
            hpc_symbols,
            transform_registry: registry,
        }
    }

    pub fn add_flux(pond: &mut FluxPond, flux_item: FluxItem) -> Result<(), anyhow::Error> {
        let thread = flux_item.thread.clone();
        let schema = match &flux_item.intent.schema {
            Binding::Bound(b) => b.clone(),
            _ => return Err(anyhow!("Unbound schema")),
        };

        let bucket = Kernel::bucket_mut(pond, &thread, &schema);
        bucket.push(flux_item);

        Ok(())
    }

    pub fn get_flux(&self, thread: &ThreadId, schema: &Schema) -> Option<&[FluxItem]> {
        self.flux_pond
            .get(thread)
            .and_then(|m| m.get(schema))
            .map(|v| v.as_slice())
    }

    pub fn find_flux_by_name(&self, name: &str) -> Option<&FluxItem> {
        for thread_map in self.flux_pond.values() {
            for bucket in thread_map.values() {
                if let Some(f) = bucket.iter().find(|f| f.name == name) {
                    return Some(f);
                }
            }
        }
        None
    }

    pub fn remove_flux_by_name(&mut self, name: &str) {
        for thread_map in self.flux_pond.values_mut() {
            for bucket in thread_map.values_mut() {
                bucket.retain(|f| f.name != name);
            }
        }
    }

    pub fn hash_flux(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();

        // 1. Sort threads
        let mut threads: Vec<&String> = self.flux_pond.keys().collect();
        threads.sort();

        for thread in threads {
            hasher.update(thread.as_bytes());

            let schema_map = &self.flux_pond[thread];

            // 2. Sort schemas within thread
            let mut schemas: Vec<&String> = schema_map.keys().collect();
            schemas.sort();

            for schema in schemas {
                hasher.update(schema.as_bytes());

                let bucket = &schema_map[schema];

                // 3. Hash all flux items in the bucket
                //    Order must be deterministic
                let mut items: Vec<&FluxItem> = bucket.iter().collect();

                // If FluxItem has a stable field (name, timestamp, correlation),
                // sort by it. Pick ONE deterministic rule.
                items.sort_by(|a, b| a.name.cmp(&b.name));

                for item in items {
                    hasher.update(&item.to_cbor());
                }
            }
        }

        hasher.finalize().into()
    }

    pub fn bucket_mut<'a>(
        pond: &'a mut FluxPond,
        thread: &ThreadId,
        schema: &Schema,
    ) -> &'a mut Vec<FluxItem> {
        pond.entry(thread.clone())
            .or_default()
            .entry(schema.clone())
            .or_default()
    }
}
