use std::collections::HashMap;

use anyhow::anyhow;
use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta, payload::FluxPayload},
    hpc::directive::{MembraneDirective, ResourceBacking},
    membrane::{ActionType, MembraneAction},
};

pub fn process_directives(
    directives: &[MembraneDirective],
    resource_map: &mut HashMap<Vec<u8>, (Vec<u8>, ResourceBacking)>,
) -> Result<Option<FluxItem>, anyhow::Error> {
    let mut membrane_actions = Vec::new();
    for directive in directives {
        match directive {
            MembraneDirective::RegisterResource {
                logical_id,
                backing,
                token,
                cause,
            } => {
                resource_map.insert(logical_id.to_vec(), (token.to_vec(), backing.clone()));
                membrane_actions.push(MembraneAction {
                    action: ActionType::RegisterResource,
                    logical_id: logical_id.to_vec(),
                    backing: backing.clone(),
                    cause: cause.clone(),
                })
            }
            _ => return Err(anyhow!("???")),
        }
    }

    let out_flux = if membrane_actions.len() > 0 {
        Some(FluxItem {
            name: "system.membrane.actions".to_string(),
            availability: FluxAvailability::Now,
            schema: Some("rhex://schema.system.membrane.actions".to_string()),
            payload: FluxPayload::Binary(serde_cbor::to_vec(&membrane_actions).unwrap()),
            meta: FluxMeta {
                creator: "membrane.macos".to_string(),
                timestamp: 0,
            },
        })
    } else {
        None
    };

    Ok(out_flux)
}
