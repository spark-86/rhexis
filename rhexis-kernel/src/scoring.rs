use crate::{FluxPond, json_path::JsonPathExt};
use rhexis_core::{
    flux::item::FluxItem,
    rhex::{intent::Binding, payload::RhexPayload},
    rhp::descriptor::{PatternDescriptor, TransformDescriptor},
};

/// Scores a single flux item against a pattern (unchanged logic)
pub fn score_pattern(pattern: &PatternDescriptor, flux_item: &FluxItem) -> usize {
    let mut score: usize = 0;

    if let Some(key) = &pattern.key {
        if &flux_item.name != key {
            return 0;
        }
        score += 10;
    }

    if let Some(schema) = &pattern.schema {
        match &flux_item.intent.schema {
            Binding::Bound(v) if v == schema => score += 1000,
            _ => return 0,
        }
    }

    if let Some(req) = &pattern.required_fields {
        if all_required_present(req, flux_item) {
            score += 100;
        } else {
            return 0;
        }
    }

    score
}

fn all_required_present(req: &[String], flux: &FluxItem) -> bool {
    match &flux.intent.data {
        RhexPayload::Json(v) => req.iter().all(|p| v.contains_path(p)),
        RhexPayload::Mixed { meta, .. } => req.iter().all(|p| meta.contains_path(p)),
        _ => false,
    }
}

pub fn score_transform(
    transform_desc: &TransformDescriptor,
    flux_pond: &FluxPond,
) -> (usize, Vec<String>, Vec<String>, Option<String>) {
    let mut score: usize = 0;

    let mut matched_flux: Vec<String> = Vec::new();
    let mut consumed_flux: Vec<String> = Vec::new();

    for interaction in &transform_desc.interacts {
        let is_required = interaction.flags.iter().any(|f| f == "required");
        let is_consumed = interaction.flags.iter().any(|f| f == "consumed");
        let is_multiple = interaction.flags.iter().any(|f| f == "multiple");

        let thread = &interaction.thread;
        let schema = match &interaction.schema {
            Some(s) => s,
            None => {
                // schema is mandatory now — descriptor error
                return (0, vec![], vec![], None);
            }
        };

        let bucket = flux_pond
            .get(thread)
            .and_then(|m| m.get(schema))
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        // ---- MULTIPLE ----
        if is_multiple {
            // zero-length is allowed
            if is_required && bucket.is_empty() {
                return (0, vec![], vec![], None);
            }

            for flux in bucket {
                let s = score_pattern(interaction, flux);
                if s > 0 {
                    score += s;
                    matched_flux.push(flux.name.clone());
                    if is_consumed {
                        consumed_flux.push(flux.name.clone());
                    }
                }
            }

            continue;
        }

        // ---- SINGLE ----
        let mut best_score = 0;
        let mut best_flux: Option<&FluxItem> = None;

        for flux in bucket {
            let s = score_pattern(interaction, flux);
            if s > best_score {
                best_score = s;
                best_flux = Some(flux);
            }
        }

        if best_score == 0 {
            if is_required {
                return (0, vec![], vec![], None);
            }
            continue;
        }

        let flux = best_flux.unwrap();
        matched_flux.push(flux.name.clone());

        if is_consumed {
            score += best_score * 10;
            consumed_flux.push(flux.name.clone());
        } else {
            score += best_score;
        }
    }

    // ---- BIND RESOLUTION ----
    let bound_flux = if let Some(bind_schema) = &transform_desc.bind {
        let exec_corr = matched_flux
            .first()
            .and_then(|name| find_flux_by_name(flux_pond, name))
            .and_then(|f| f.correlation);

        resolve_bind(bind_schema, flux_pond, &exec_corr)
    } else {
        None
    };

    if transform_desc.bind.is_some() && bound_flux.is_none() {
        return (0, vec![], vec![], None);
    }

    (score, matched_flux, consumed_flux, bound_flux)
}

/// Bind now searches *by schema + correlation*, not by matched names
fn resolve_bind(
    bind_schema: &str,
    flux_pond: &FluxPond,
    exec_corr: &Option<[u8; 32]>,
) -> Option<String> {
    let corr = match exec_corr {
        Some(c) => *c,
        None => return None,
    };

    for thread_map in flux_pond.values() {
        if let Some(bucket) = thread_map.get(bind_schema) {
            for flux in bucket {
                if flux.correlation == Some(corr) {
                    return Some(flux.name.clone());
                }
            }
        }
    }

    None
}

/// Debug / tooling helper only
fn find_flux_by_name<'a>(pond: &'a FluxPond, name: &str) -> Option<&'a FluxItem> {
    for thread_map in pond.values() {
        for bucket in thread_map.values() {
            if let Some(f) = bucket.iter().find(|f| f.name == name) {
                return Some(f);
            }
        }
    }
    None
}
