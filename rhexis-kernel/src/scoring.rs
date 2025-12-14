use std::collections::HashMap;

use crate::json_path::JsonPathExt;
use rhexis_core::{
    flux::{item::FluxItem, payload::FluxPayload},
    rhp::descriptor::{PatternDescriptor, TransformDescriptor},
};

pub fn score_pattern(pattern: &PatternDescriptor, flux_item: &FluxItem) -> usize {
    let mut score: usize = 0;

    if pattern.key.is_some() {
        if flux_item.name != pattern.key.clone().unwrap() {
            return 0;
        } else {
            score += 10;
        }
    }
    if pattern.schema.is_some() {
        if flux_item.schema != pattern.schema.clone() {
            return 0;
        } else {
            score += 1000;
        }
    }
    if pattern.required_fields.is_some() {
        if all_required_present(&pattern.required_fields.clone().unwrap(), flux_item) {
            score += 100;
        } else {
            return 0;
        }
    }
    score
}

fn all_required_present(req: &[String], flux: &FluxItem) -> bool {
    match &flux.payload {
        FluxPayload::Json(v) => req.iter().all(|p| v.contains_path(p)),
        FluxPayload::Mixed { meta, data } => {
            let _ = data;
            req.iter().all(|p| meta.contains_path(p))
        }
        FluxPayload::Binary(_) | FluxPayload::None => false,
    }
}

pub fn score_transform(
    transform_desc: &TransformDescriptor,
    flux_pond: &HashMap<String, FluxItem>,
) -> (usize, Vec<String>, Vec<String>) {
    let mut score: usize = 0;

    // Flux that have been matched already (any role)
    let mut used_flux: Vec<String> = Vec::new();

    // Ordered list of matched flux, same order as interacts[]
    let mut matched_flux: Vec<String> = Vec::new();

    // Subset: which of the matched flux were consumed
    let mut consumed_flux: Vec<String> = Vec::new();

    for interaction in transform_desc.interacts.iter() {
        let is_required = interaction.flags.iter().any(|f| f == "required");
        let is_consumed = interaction.flags.iter().any(|f| f == "consumed");

        let mut best_score = 0;
        let mut best_name: Option<String> = None;

        for flux_item in flux_pond.values() {
            if used_flux.contains(&flux_item.name) {
                continue;
            }

            let flux_score = score_pattern(&interaction, flux_item);
            if flux_score > best_score {
                best_score = flux_score;
                best_name = Some(flux_item.name.clone());
            }
        }

        // Required interaction not satisfied → reject transform
        if best_score == 0 {
            if is_required {
                return (0, vec![], vec![]);
            } else {
                // Optional interaction missing: preserve positional intent
                // (push nothing, score nothing)
                continue;
            }
        }

        let flux_name = best_name.unwrap();

        // Weighting stays the same as before
        if is_consumed {
            score += best_score * 10;
            consumed_flux.push(flux_name.clone());
        } else {
            score += best_score;
        }

        used_flux.push(flux_name.clone());
        matched_flux.push(flux_name);
    }

    (score, matched_flux, consumed_flux)
}
