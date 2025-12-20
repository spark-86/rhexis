use std::sync::Arc;

use rhexis_core::{
    flux::item::FluxItem, registry::LoadedTransform, transform::context::TransformContext,
};

use crate::{ExecutionArtifacts, ScoreResult};

use super::Kernel;

impl Kernel {
    pub fn execute_single(
        &self,
        transform: &Arc<LoadedTransform>,
        score: ScoreResult,
        artifacts: &mut ExecutionArtifacts,
    ) {
        // ---- materialize matched flux ----
        let mut total_flux: Vec<FluxItem> = Vec::new();

        for name in &score.matched {
            let flux = self
                .find_flux_by_name(name)
                .expect("matched flux disappeared from pond")
                .clone();
            total_flux.push(flux);
        }

        if let Some(bound) = &score.bound {
            let flux = self
                .find_flux_by_name(bound)
                .expect("bound flux disappeared from pond")
                .clone();
            total_flux.push(flux);
        }

        if total_flux.is_empty() {
            return;
        }

        let parent_corr = total_flux[0].correlation;

        // ---- execute transform ----
        let mut ctx = TransformContext {
            input: &serde_cbor::to_vec(&total_flux).unwrap(),
            output: &mut None,
            diag: &mut None,
            hpc_calls: &mut None,
        };

        let result = (transform.entry.entry)(&mut ctx);
        if result != 0 {
            return;
        }

        self.handle_outputs(transform, score.score, parent_corr, &mut ctx, artifacts);
    }
}
