use rhexis_core::{
    flux::item::FluxItem,
    membrane::{CauseHeader, HpcCall},
    rhex::payload::RhexPayload,
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let intput: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut hpc_call_list = Vec::new();

    for item in intput {
        let payload = match item.intent.data {
            RhexPayload::Mixed { meta: _, data } => data,
            _ => return -1,
        };

        let cause = CauseHeader {
            target: format!("crypto.ed25519.signed.{}", hex::encode(item.intent.nonce)),
            thread: "crypto.ed25519.signed".to_string(),
            schema: "rhex://schema.crypto.ed25519.signed".to_string(),
            payload: vec![],
        };

        let hpc_call = HpcCall {
            name: "crypto.ed25519.sign".to_string(),
            thread: "crypto.ed25519.sign".to_string(),
            logical_id: Some(item.intent.nonce.to_vec()),
            token: None,
            input: serde_cbor::to_vec(&payload).unwrap(),
            cause: Some(serde_cbor::to_vec(&cause).unwrap()),
            correlation: None,
        };

        hpc_call_list.push(hpc_call);
    }

    *ctx.hpc_calls = Some(serde_cbor::to_vec(&hpc_call_list).unwrap());

    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
