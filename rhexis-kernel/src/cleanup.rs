use std::collections::HashMap;

use rhexis_core::flux::item::FluxItem;

use super::Kernel;

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

    /// Materialize winning flux into the pond
    pub fn materialize(&mut self, collapse: HashMap<String, (FluxItem, usize)>) {
        for (_, (item, _score)) in collapse {
            // Check to make sure we aren't overflowing the thread
            if self.thread_pressure.get(&item.thread).unwrap().clone() > 0 {
                // Thread is currently overloaded, add to overflow bucket
                self.overflow
                    .entry(item.thread.clone())
                    .or_default()
                    .push(item.clone());
                continue;
            } else {
                // Thread is not overloaded and can be added to
                Kernel::add_flux(&mut self.flux_pond, item).expect("failed to materialize flux");
            }
        }
    }

    pub fn prune_empty(&mut self) {
        self.flux_pond.retain(|_, schema_map| {
            schema_map.retain(|_, bucket| !bucket.is_empty());
            !schema_map.is_empty()
        });
    }
}
