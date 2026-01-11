use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
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

    let (_, rhex) = match &input[0].intent.data {
        RhexPayload::Mixed { meta, data } => {
            (meta, serde_cbor::from_slice::<Rhex>(&data[0]).unwrap())
        }
        _ => return -1,
    };

    let mut out_intent = RhexIntent::new(RhexIntent::gen_nonce());
    out_intent.schema = Binding::Bound("rhex://schema.ui.output".to_string());
    out_intent.data = RhexPayload::Json(json!({
        "magic": URL_SAFE_NO_PAD.encode(rhex.magic),
        "intent": rhex.intent,
        "context": rhex.context,
        "signatures": rhex.signatures,
        "current_hash": URL_SAFE_NO_PAD.encode(rhex.current_hash.unwrap())
    }));

    transform_output.push(FluxItem {
        name: format!("ui.output.{}", hex::encode(rhex.current_hash.unwrap())),
        thread: "ui.output".to_string(),
        availability: FluxAvailability::Now,
        intent: out_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.rhex.summarize".to_string(),
            timestamp: 0,
        },
    });

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
