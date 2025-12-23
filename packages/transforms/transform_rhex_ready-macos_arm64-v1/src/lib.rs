use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{intent::RhexIntent, payload::RhexPayload},
    transform::{context::TransformContext, entry::TransformEntry},
};
use struct_rhex::{RhexCommited, RhexDone};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    // Input is a schema.rhex.committed
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let payload = match &input[0].intent.data {
        RhexPayload::Json(j) => j,
        _ => return -1,
    };
    let commited = serde_json::from_value::<RhexCommited>(payload.clone()).unwrap();

    let mut intent = RhexIntent::new(RhexIntent::gen_nonce());

    let done = RhexDone {
        current_hash: commited.current_hash.clone(),
        timestamp: 0,
        scope: commited.scope.clone(),
    };

    intent.data = RhexPayload::Json(serde_json::to_value(done).unwrap());

    let out_flux = vec![FluxItem {
        name: format!("rhex.ready.{}", hex::encode(&commited.current_hash)).to_string(),
        thread: "flux".to_string(),
        availability: FluxAvailability::Now,
        intent,
        correlation: input[0].correlation.clone(),
        meta: FluxMeta {
            creator: "transform.rhex.ready".to_string(),
            timestamp: 0,
        },
    }];

    *ctx.output = Some(serde_cbor::to_vec(&out_flux).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
