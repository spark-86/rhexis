use std::collections::HashMap;

use crate::json_path::JsonPathExt;
use rhexis_core::{
    flux::{item::FluxItem, payload::FluxPayload},
    transform::{pattern::TransformPattern, signature::TransformSignature},
};

pub fn score_pattern(pattern: &TransformPattern, flux_item: &FluxItem) -> usize {
    let mut score: usize = 0;

    if pattern.key.is_some() {
        if flux_item.name != pattern.key.clone().unwrap() {
            return 0;
        } else {
            score += 10;
        }
    }
    if pattern.schema.is_some() {
        if flux_item.schema != pattern.schema.clone().unwrap() {
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
    transform_sig: &TransformSignature,
    flux_pond: &HashMap<String, FluxItem>,
) -> (usize, Vec<String>, Vec<String>) {
    let mut score: usize = 0;
    let mut consume_used = Vec::new();
    for consumed in transform_sig.consumes.iter() {
        let mut pattern_score = 0;
        let mut used_name = "".to_string();
        for flux_item in flux_pond.values() {
            if consume_used.contains(&flux_item.name.to_string()) {
                continue;
            }

            let flux_score = score_pattern(&consumed, flux_item);
            if flux_score > pattern_score {
                pattern_score = flux_score;
                used_name = flux_item.name.to_string();
            }
        }
        if pattern_score == 0 {
            return (0, vec![], vec![]);
        }
        score += pattern_score * 10;
        consume_used.push(used_name);
    }
    let mut observe_used = Vec::new();

    for observed in transform_sig.observes.iter() {
        let mut pattern_score = 0;
        let mut used_name = "".to_string();
        for flux_item in flux_pond.values() {
            if consume_used.contains(&flux_item.name.to_string())
                || observe_used.contains(&flux_item.name.to_string())
            {
                continue;
            }

            let flux_score = score_pattern(&observed, flux_item);
            if flux_score > pattern_score {
                pattern_score = flux_score;
                used_name = flux_item.name.to_string();
            }
        }
        if pattern_score == 0 {
            return (0, vec![], vec![]);
        }
        score += pattern_score;
        observe_used.push(used_name);
    }
    (score, observe_used, consume_used)
}
