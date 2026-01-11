use std::collections::HashMap;

use rhexis_core::{flux::item::FluxItem, rhex::intent::Binding};

use super::Kernel;

pub const PRESSURE_LEVEL: usize = 64;
pub const HARD_CAP: usize = 500;

impl Kernel {
    /// Consume specific flux items by name (rare, but explicit)
    pub fn consume_flux(&mut self, consumed: Vec<String>) {
        for thread_map in self.flux_pond.values_mut() {
            for bucket in thread_map.values_mut() {
                bucket.retain(|f| !consumed.contains(&f.name));
            }
        }
    }

    /// Detonate all flux that share any of the given correlations
    pub fn detonate(&mut self, detonators: Vec<[u8; 32]>) {
        for thread_map in self.flux_pond.values_mut() {
            for bucket in thread_map.values_mut() {
                bucket.retain(|f| match f.correlation {
                    Some(c) => !detonators.contains(&c),
                    None => true,
                });
            }
        }
    }

    pub fn materialize(
        &mut self,
        collapse: HashMap<String, (FluxItem, usize)>,
    ) -> Result<Vec<FluxItem>, anyhow::Error> {
        let mut tossed_flux: Vec<FluxItem> = Vec::new();

        // Local helper: count *items* in a thread, not schema buckets.
        let thread_item_count =
            |pond: &HashMap<String, HashMap<String, Vec<FluxItem>>>, thread: &str| -> usize {
                pond.get(thread)
                    .map(|schema_map| schema_map.values().map(|bucket| bucket.len()).sum())
                    .unwrap_or(0)
            };

        // Local helper: drain overflow into pond up to a target count.
        // NOTE: This is "backfill", not "relieve pressure".
        let backfill_from_overflow = |kernel: &mut Kernel, thread: &str| {
            let current = thread_item_count(&kernel.flux_pond, thread);
            if current >= PRESSURE_LEVEL {
                return;
            }

            let need = PRESSURE_LEVEL - current;

            if let Some(overflow) = kernel.overflow.get_mut(thread) {
                if overflow.is_empty() || need == 0 {
                    return;
                }

                println!(
                    "Backfilling thread {} from overflow: up to {} items",
                    thread, need
                );

                // Drain up to `need` items from overflow and push them into pond by schema.
                let drain_n = need.min(overflow.len());
                let drained = overflow.drain(0..drain_n);

                let pond_thread = kernel
                    .flux_pond
                    .entry(thread.to_string())
                    .or_insert_with(HashMap::new);

                for drained_item in drained {
                    let schema = match &drained_item.intent.schema {
                        Binding::Bound(v) => v.clone(),
                        Binding::Unbound => continue,
                    };
                    pond_thread
                        .entry(schema)
                        .or_insert_with(Vec::new)
                        .push(drained_item);
                }
            }
        };

        for (_, (item, _score)) in collapse {
            let thread = item.thread.clone();

            // Hard cap check based on *items*.
            let count = thread_item_count(&self.flux_pond, &thread);
            if count >= HARD_CAP {
                tossed_flux.push(item);
                continue;
            }

            // If thread is currently overloaded, do NOT add to pond. Put the incoming item in overflow.
            let pressure = *self.thread_pressure.get(&thread).unwrap_or(&0);
            if pressure > 0 {
                println!(
                    "Thread {} is overloaded; sending winning item to overflow",
                    &thread
                );
                self.overflow.entry(thread.clone()).or_default().push(item);
                // Do not backfill while overloaded; that's semantically backwards.
                continue;
            }

            // Thread is not overloaded: add the winning item to pond.
            Kernel::add_flux(&mut self.flux_pond, item).expect("failed to materialize flux");

            // After admitting, optionally backfill from overflow up to PRESSURE_LEVEL (still not overloaded).
            // This keeps the pond "warm" without pretending it changes pressure.
            backfill_from_overflow(self, &thread);

            // Optional: prune empty overflow buckets for tidiness (no correctness impact)
            if let Some(v) = self.overflow.get(&thread) {
                if v.is_empty() {
                    self.overflow.remove(&thread);
                }
            }
        }

        Ok(tossed_flux)
    }

    pub fn prune_empty(&mut self) {
        self.flux_pond.retain(|_, schema_map| {
            schema_map.retain(|_, bucket| !bucket.is_empty());
            !schema_map.is_empty()
        });
    }
}
