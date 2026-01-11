use std::collections::HashMap;

use rhexis_core::{
    flux::{item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde_json::json;
use struct_lattice::LatticeScopeLocation;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();
    let mut requests: Vec<FluxItem> = Vec::new();
    let mut table: HashMap<String, LatticeScopeLocation> = HashMap::new();

    for item in input {
        if item.intent.schema == Binding::Bound("rhex://schema.lattice.scope.lookup".to_string()) {
            requests.push(item);
        } else {
            let data_bin = match item.intent.data {
                RhexPayload::Binary { data } => data,
                _ => return -1,
            };

            table = serde_cbor::from_slice(&data_bin).unwrap();
        }
    }
    for request in requests {
        let local_entry = table.get(&request.name);

        let mut out_intent: RhexIntent = RhexIntent::new(RhexIntent::gen_nonce());
        if local_entry.is_some() {
            let entry = local_entry.unwrap();
            out_intent.schema =
                Binding::Bound("rhex://schema.lattice.scope.lookup.result".to_string());
            out_intent.data = RhexPayload::Json(json!({
                "entry": entry.clone(),
                "status": "complete",
            }));
            transform_output.push(FluxItem {
                name: format!("lattice.scope.lookup.result.{}", hex::encode(&request.name)),
                thread: "lattice.scope.lookup.result".to_string(),
                availability: request.availability,
                intent: out_intent,
                correlation: request.correlation,
                meta: FluxMeta {
                    creator: "transform.lattice.scope.lookup".to_string(),
                    timestamp: 0,
                },
            });
        } else {
            // Submit remote request here.
        }
    }

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
