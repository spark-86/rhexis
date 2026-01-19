use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde_json::json;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    for item in input {
        let (meta, bin) = match &item.intent.data {
            RhexPayload::Mixed { meta, data } => (meta, data),
            _ => return -1,
        };

        let mut put_intent = RhexIntent::new(RhexIntent::gen_nonce());
        put_intent.schema = Binding::Bound("rhex://schema.system.registry.action".to_string());
        put_intent.data = RhexPayload::Mixed {
            meta: json!({
                "action": meta["action"],
                "data_type": meta["data_type"]
            }),
            data: vec![bin[0].clone()],
        };
        transform_output.push(FluxItem {
            name: format!("system.registry.action.{}", hex::encode(&put_intent.nonce)),
            thread: "system.registry.actions".to_string(),
            availability: FluxAvailability::Now,
            intent: put_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.system.registry.add".to_string(),
                timestamp: 0,
            },
        })
    }

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
