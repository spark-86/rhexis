use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta, payload::FluxPayload},
    membrane::HpcCall,
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleWriteOp {
    bytes: Vec<u8>,
}

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let flux: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let bytes = flux[0].payload.as_bytes();
    let op = ConsoleWriteOp {
        bytes: bytes.to_vec(),
    };
    let encoded = serde_cbor::to_vec(&op).unwrap();

    let out_hpc = vec![HpcCall {
        name: "io.console.write".to_string(),
        logical_id: None,
        token: None,
        input: encoded.clone(),
        cause: None,
    }];

    *ctx.hpc_calls = Some(serde_cbor::to_vec(&out_hpc).unwrap());

    let out_flux = vec![FluxItem {
        name: "_console.write".to_string(),
        availability: FluxAvailability::Now,
        schema: None,
        payload: FluxPayload::Binary(encoded),
        meta: FluxMeta {
            creator: "transform.console.write".to_string(),
            timestamp: 0,
        },
    }];

    *ctx.output = Some(serde_cbor::to_vec(&out_flux).unwrap());
    // return
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
