use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        Rhex,
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

    let rhex: Rhex = match &input[1].intent.data {
        RhexPayload::Mixed { meta: _, data } => serde_cbor::from_slice(&data[0]).unwrap(),
        _ => return -1,
    };

    let mut out_intent = RhexIntent::new(RhexIntent::gen_nonce());
    out_intent.schema = Binding::Bound("rhex://schema.rhex.store.confirmed".to_string());
    out_intent.data = RhexPayload::Mixed {
        meta: json!({}),
        data: vec![serde_cbor::to_vec(&rhex).unwrap()],
    };

    transform_output.push(FluxItem {
        name: format!(
            "rhex.store.complete.{}",
            hex::encode(rhex.current_hash.unwrap())
        ),
        thread: "rhex.store.complete".to_string(),
        availability: FluxAvailability::Now,
        intent: out_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.rhex.store.confirm".to_string(),
            timestamp: 0,
        },
    });

    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
