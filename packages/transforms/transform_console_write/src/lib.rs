use rhexis_core::{
    flux::{item::FluxItem, meta::FluxMeta, payload::FluxPayload},
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleWriteOp {
    bytes: Vec<u8>,
}

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let flux = unsafe { (*ctx).input.first().unwrap() };
    let bytes = flux.payload.as_bytes();
    let op = ConsoleWriteOp {
        bytes: bytes.to_vec(),
    };
    let encoded = serde_cbor::to_vec(&op).unwrap();

    unsafe {
        (*ctx)
            .hpc_calls
            .push(("io.console.write".to_string(), encoded.clone()));

        (*ctx).output.push(FluxItem {
            name: "_console.write".to_string(),
            schema: None,
            payload: FluxPayload::Binary(encoded),
            meta: FluxMeta {
                creator: "transform.console.write".to_string(),
                timestamp: 0,
            },
        });
    }

    // return
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
