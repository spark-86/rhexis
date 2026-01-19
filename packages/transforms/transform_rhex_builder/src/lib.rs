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

/// Dealing with keys is garbage. Like literally.
///
/// Json {
///     "scope": "...",
///     "record_type": "...",
///     "usher_key_present": true/false
/// }
///
///
/// Data 0: previous_hash
/// Data 1: nonce
/// Data 2: author_public_key
/// Data 3 (optional): usher_public_key
/// Data 4: data - CBOR(RhexPayload)
///

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

        let mut out_intent = RhexIntent::new(RhexIntent::gen_nonce());
        out_intent.schema = Binding::Bound("rhex://schema.rhex.intent".to_string());
        let mut rhex = Rhex::new();
        rhex.intent.previous_hash = Binding::Bound(bin[0].clone().try_into().unwrap());
        rhex.intent.scope =
            Binding::Bound(meta.get("scope").unwrap().as_str().unwrap().to_string());
        rhex.intent.nonce = bin[1].clone().try_into().unwrap();
        rhex.intent.author_public_key = Binding::Bound(bin[2].clone().try_into().unwrap());
        if meta["usher_key_present"].as_bool().unwrap() {
            rhex.intent.usher_public_key = Binding::Bound(bin[3].clone().try_into().unwrap());
            rhex.intent.data = serde_cbor::from_slice(&bin[4].clone()).unwrap();
        } else {
            rhex.intent.data = serde_cbor::from_slice(&bin[3].clone()).unwrap();
        }
        rhex.intent.record_type = Binding::Bound(meta["record_type"].to_string());

        out_intent.data = RhexPayload::Mixed {
            meta: json!({}),
            data: vec![serde_cbor::to_vec(&rhex).unwrap()],
        };

        transform_output.push(FluxItem {
            name: format!("rhex.intent.{}", hex::encode(&out_intent.nonce)),
            thread: "rhex.intent".to_string(),
            availability: FluxAvailability::Now,
            intent: out_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.rhex.intent".to_string(),
                timestamp: 0,
            },
        });
    }
    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
