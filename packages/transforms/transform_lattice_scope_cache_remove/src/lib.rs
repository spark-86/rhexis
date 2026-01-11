use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use struct_lattice::LatticeScopeCacheAction;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    for item in input {
        let action: LatticeScopeCacheAction = match &item.intent.data {
            RhexPayload::Json(j) => serde_json::from_value(j.clone()).unwrap(),
            _ => return -1,
        };

        match &action {
            LatticeScopeCacheAction::Remove(_) => {
                let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
                intent.schema =
                    Binding::Bound("rhex://schema.lattice.scope.cache.action".to_string());
                intent.data = RhexPayload::Json(serde_json::to_value(action).unwrap());
                transform_output.push(FluxItem {
                    name: format!("lattice.scope.cache.action.{}", hex::encode(intent.nonce)),
                    thread: "lattice.scope.cache.queue".to_string(),
                    availability: FluxAvailability::Now,
                    intent,
                    correlation: None,
                    meta: FluxMeta {
                        creator: "transform.lattice.scope.cache.remove".to_string(),
                        timestamp: 0,
                    },
                });
            }
            _ => return -2,
        }
    }

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
