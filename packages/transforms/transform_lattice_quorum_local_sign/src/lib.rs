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
        let (key, signing_hash) = match &item.intent.data {
            RhexPayload::Mixed { meta: _, data } => (&data[0][..], &data[1][..]),
            _ => return -1,
        };

        let mut out_intent = RhexIntent::new(RhexIntent::gen_nonce());
        out_intent.schema = Binding::Bound("rhex://schema.crypto.ed25519.sign".to_string());
        out_intent.data = RhexPayload::Mixed {
            meta: json!({}),
            data: vec![key.to_vec(), signing_hash.to_vec()],
        };
        transform_output.push(FluxItem {
            name: format!("crypto.ed25519.sign.{}", hex::encode(item.intent.nonce)),
            thread: "crypto.ed25519.sign".to_string(),
            availability: FluxAvailability::Now,
            intent: out_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.lattice.quorum.local.sign".to_string(),
                timestamp: 0,
            },
        })
    }
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
