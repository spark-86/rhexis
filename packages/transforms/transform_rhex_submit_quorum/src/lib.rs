use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{Rhex, intent::RhexIntent, payload::RhexPayload},
    transform::{context::TransformContext, entry::TransformEntry},
};
use struct_lattice::usher::Usher;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();
    let (meta, data) = match &input[0].intent.data {
        RhexPayload::Mixed { meta, data } => (meta, data),
        _ => return -1,
    };
    let rhex: Rhex = serde_cbor::from_slice(&data[0]).unwrap();
    let ushers: Vec<Usher> = match &input[1].intent.data {
        RhexPayload::Json(ushers) => serde_json::from_value(ushers.clone()).unwrap(),
        _ => return -2,
    };

    let new_meta = serde_json::json!({
        "prev_meta": meta,
        "scope": rhex.intent.scope.clone(),
        "ushers": ushers,
    });

    let mut awaiting_intent = RhexIntent::new(RhexIntent::gen_nonce());
    awaiting_intent.data = RhexPayload::Mixed {
        meta: new_meta,
        data: vec![rhex.as_bytes()],
    };
    transform_output.push(FluxItem {
        name: format!(
            "rhex.awaiting_quorum.{}",
            hex::encode(rhex.current_hash.unwrap())
        ),
        thread: "rhex.quorum".to_string(),
        availability: FluxAvailability::Now,
        intent: awaiting_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.rhex.submit.quorum".to_string(),
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
