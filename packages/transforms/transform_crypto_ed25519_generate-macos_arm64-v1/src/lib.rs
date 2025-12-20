use rand::random;
use rhexis_core::{
    flux::item::FluxItem,
    membrane::{CauseHeader, HpcCall},
    rhex::payload::RhexPayload,
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let id: [u8; 32] = random();
    let out_hpc = vec![HpcCall {
        name: "crypto.ed25519.generate".to_string(),
        logical_id: Some(id.clone().to_vec()),
        thread: input[0].thread.clone(),
        token: None,
        input: vec![],
        cause: Some(
            serde_cbor::to_vec(&CauseHeader {
                target: format!("crypto.ed25519.generate.complete.{}", &hex::encode(id))
                    .to_string(),
                thread: "crypto_engine".to_string(),
                schema: "rhex://schema.crypto.ed25519.generate.complete".to_string(),
                payload: serde_cbor::to_vec(&RhexPayload::Mixed {
                    meta: serde_json::json!({
                        "slot0": "logical_id"
                    }),
                    data: vec![id.to_vec()],
                })
                .unwrap(),
            })
            .unwrap(),
        ),
        correlation: input[0].correlation.clone(),
    }];
    *ctx.hpc_calls = Some(serde_cbor::to_vec(&out_hpc).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
