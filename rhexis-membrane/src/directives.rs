use std::collections::HashMap;

use anyhow::anyhow;
use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    hpc::directive::{MembraneDirective, ResourceBacking},
    membrane::{ActionType, CauseHeader, MembraneAction},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
};

pub fn process_directives(
    directives: &[MembraneDirective],
    resource_map: &mut HashMap<Vec<u8>, (Vec<u8>, ResourceBacking)>,
) -> Result<Option<Vec<FluxItem>>, anyhow::Error> {
    let mut membrane_actions = Vec::new();
    let mut smuggled = Vec::new();
    for directive in directives {
        match directive {
            MembraneDirective::RegisterResource {
                logical_id,
                backing,
                token,
                cause,
                correlation,
            } => {
                println!("What we got: {:?}", &correlation);
                resource_map.insert(logical_id.to_vec(), (token.to_vec(), backing.clone()));
                membrane_actions.push(MembraneAction {
                    action: ActionType::RegisterResource,
                    logical_id: logical_id.to_vec(),
                    backing: backing.clone(),
                    cause: cause.clone(),
                    correlation: correlation.clone(),
                });
                if cause.is_some() {
                    let chopped_cause: CauseHeader =
                        serde_cbor::from_slice(&cause.clone().unwrap()).unwrap();
                    let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
                    intent.schema = Binding::Bound(chopped_cause.schema);
                    intent.data = serde_cbor::from_slice(&chopped_cause.payload.clone()).unwrap();
                    smuggled.push(FluxItem {
                        name: chopped_cause.target.clone(),
                        thread: chopped_cause.thread.clone(),
                        availability: FluxAvailability::Now,
                        intent,
                        correlation: correlation.clone(),
                        meta: FluxMeta {
                            creator: "system.membrane".to_string(),
                            timestamp: 0,
                        },
                    })
                }
            }
            _ => return Err(anyhow!("???")),
        }
    }

    let out_flux = if membrane_actions.len() > 0 {
        /*Some(FluxItem {
            name: "system.membrane.actions".to_string(),
            thread: Some("system".to_string()),
            availability: FluxAvailability::Now,
            intent,
            correlation: None,
            meta: FluxMeta {
                creator: "membrane.macos".to_string(),
                timestamp: 0,
            },
        })*/
        let mut flux_load = Vec::new();
        for action in &membrane_actions {
            let thread = "system.membrane".to_string();
            let logical_id = action.logical_id.clone();
            let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
            intent.schema = Binding::Bound("rhex://schema.system.membrane.action".to_string());
            intent.data = RhexPayload::None;
            flux_load.push(FluxItem {
                name: format!("system.membrane.action.{}", hex::encode(logical_id)).to_string(),
                thread,
                availability: FluxAvailability::Now,
                intent,
                correlation: action.correlation,
                meta: FluxMeta {
                    creator: "system.membrane".to_string(),
                    timestamp: 0,
                },
            })
        }
        Some(flux_load)
    } else {
        None
    };
    if out_flux.is_some() {
        smuggled.append(&mut out_flux.clone().unwrap());
    }
    if smuggled.len() > 0 {
        Ok(Some(smuggled))
    } else {
        Ok(None)
    }
}
