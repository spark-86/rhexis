use rhexis_core::{flux::item::FluxItem, membrane::HpcCall};

use super::Kernel;

impl Kernel {
    pub fn resolve(&mut self, system_flux: Vec<FluxItem>) -> Vec<HpcCall> {
        self.ingest_flux(system_flux);

        let scores = self.score_transforms();
        let artifacts = self.execute_transforms(scores);

        self.consume_flux(artifacts.consumed);
        self.detonate(artifacts.detonators);
        self.materialize(artifacts.collapse_map);

        artifacts.hpc_calls
    }
}
