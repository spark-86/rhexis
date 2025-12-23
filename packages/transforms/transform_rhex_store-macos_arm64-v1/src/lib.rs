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
    let (rhex_meta, rhex_data) = match &input[0].intent.data {
        RhexPayload::Mixed { meta, data } => (meta, data),
        _ => return -1,
    };
    let logical_id = rhex_data[0].clone();
    let blob = rhex_data[1].clone();

    let scope = match &input[1].intent.data {
        RhexPayload::Json(j) => j,
        _ => return -1,
    };

    let destination = if scope.get("local").unwrap().as_bool().unwrap() {
        "local".to_string();
    } else {
        "remote".to_string();
    };

    let mut store_intent = RhexIntent::new(RhexIntent::gen_nonce());
    store_intent.schema = Binding::Bound("rhex://schema.data.put".to_string());
    store_intent.data = RhexPayload::Mixed {
        meta: json!({
            "constraints": [
                "disk",
                destination.clone(),
            ],
        }),
        data: vec![logical_id.clone(), blob.clone()],
    };

    let mut commit_intent = RhexIntent::new(RhexIntent::gen_nonce());
    commit_intent.schema = Binding::Bound("rhex://schema.rhex.commit".to_string());
    commit_intent.data = RhexPayload::Json(rhex_meta.clone());

    let out_flux = vec![
        FluxItem {
            name: format!("data.put.disk.{}", hex::encode(&logical_id)),
            thread: "data.put".to_string(),
            availability: FluxAvailability::Soon,
            intent: store_intent,
            correlation: input[0].correlation.clone(),
            meta: FluxMeta {
                creator: "transform.rhex.store".to_string(),
                timestamp: 0,
            },
        },
        FluxItem {
            name: format!("rhex.commit.{}", hex::encode(&logical_id)),
            thread: "rhex".to_string(),
            availability: FluxAvailability::Now,
            intent: commit_intent,
            correlation: input[0].correlation.clone(),
            meta: FluxMeta {
                creator: "transform.rhex.store".to_string(),
                timestamp: 0,
            },
        },
    ];

    *ctx.output = Some(serde_cbor::to_vec(&out_flux).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
