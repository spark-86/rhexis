use rhexis_core::{
    flux::item::FluxItem,
    membrane::{CauseHeader, HpcCall},
    rhex::payload::RhexPayload,
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde_json::json;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let intput: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();

    let corr = intput[0].correlation.clone();
    let thread = intput[0].thread.clone();
    let payload: RhexPayload = intput[0].intent.data.clone();
    let logical_id = match &payload {
        RhexPayload::Mixed { meta, data } => {
            let slot = meta["slot0"].as_str().unwrap();
            match slot {
                "logical_id" => Some(data[0].clone()),
                _ => None,
            }
        }
        _ => None,
    };

    let mut cause = CauseHeader {
        target: "crypto.ed25519.sign.complete".to_string(),
        thread: "crypto".to_string(),
        schema: "rhex://schema.crypto.ed25519.sign.complete".to_string(),
        payload: serde_cbor::to_vec(&RhexPayload::None).unwrap(),
    };
    cause.payload = if logical_id.is_some() {
        serde_cbor::to_vec(&RhexPayload::Mixed {
            meta: json!({
                "slot0": "logical_id"
            }),
            data: vec![logical_id.clone().unwrap()],
        })
        .unwrap()
    } else {
        serde_cbor::to_vec(&RhexPayload::None).unwrap()
    };

    let hpc_call = HpcCall {
        name: "crypto.ed25519.sign".to_string(),
        thread,
        logical_id,
        token: None,
        input: serde_cbor::to_vec(&payload).unwrap(),
        cause: Some(serde_cbor::to_vec(&cause).unwrap()),
        correlation: corr,
    };

    *ctx.hpc_calls = Some(serde_cbor::to_vec(&[hpc_call]).unwrap());

    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
