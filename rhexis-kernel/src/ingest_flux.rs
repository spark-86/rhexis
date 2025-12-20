use std::collections::HashMap;

use rhexis_core::flux::item::FluxItem;

use super::Kernel;

use crate::{ScoreResult, scoring};

impl Kernel {
    pub fn ingest_flux(&mut self, incoming: Vec<FluxItem>) {
        for flux in incoming {
            let result = Kernel::add_flux(&mut self.flux_pond, flux.clone());
            match result {
                Err(v) => println!("{:?}", v),
                _ => {}
            }
        }
        println!("{}", self.dump_flux_tree());
    }

    pub fn score_transforms(&self) -> HashMap<String, ScoreResult> {
        let mut map = HashMap::new();

        for transform in self.transform_registry.values() {
            let (score, matched, consumed, bound) =
                scoring::score_transform(&transform.descriptor, &self.flux_pond);

            if score == 0 {
                continue;
            }

            map.insert(
                transform.descriptor.name.clone(),
                ScoreResult {
                    score,
                    matched,
                    consumed,
                    bound,
                },
            );
        }

        map
    }
}
