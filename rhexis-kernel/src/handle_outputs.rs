use rhexis_core::{
    flux::item::FluxItem, membrane::HpcCall, registry::LoadedTransform,
    transform::context::TransformContext,
};

use crate::{ExecutionArtifacts, handle_corr, trans_output};

use super::Kernel;

impl Kernel {
    pub fn handle_outputs(
        &self,
        transform: &LoadedTransform,
        score: usize,
        parent_corr: Option<[u8; 32]>,
        ctx: &mut TransformContext,
        artifacts: &mut ExecutionArtifacts,
    ) {
        if let Some(out) = ctx.output.take() {
            let flux: Vec<FluxItem> = serde_cbor::from_slice(&out).unwrap();

            for mut item in flux {
                if let Some(effect) =
                    trans_output::check_effects(transform.descriptor.effects.clone(), item.clone())
                {
                    if effect.flags.contains(&"detonate".to_string()) {
                        artifacts.detonators.push(parent_corr.clone().unwrap());
                    }

                    handle_corr::update_correlation(&mut item, &effect, parent_corr.clone());

                    match artifacts.collapse_map.get(&item.name) {
                        Some((_, old)) if *old >= score => {}
                        _ => {
                            artifacts
                                .collapse_map
                                .insert(item.name.clone(), (item, score));
                        }
                    }
                }
            }
        }

        if let Some(diag) = ctx.diag.take() {
            artifacts.diag.extend(diag);
        }

        if let Some(calls) = ctx.hpc_calls.take() {
            let exploded: Vec<HpcCall> = serde_cbor::from_slice(&calls).unwrap();

            for call in exploded {
                artifacts.hpc_calls.push(HpcCall {
                    correlation: parent_corr.clone(),
                    ..call
                });
            }
        }
    }
}
