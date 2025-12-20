use rhexis_core::{
    flux::item::FluxItem,
    membrane::HpcCall,
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let data = input[0].intent.data.as_bytes();

    let out_hpc = vec![HpcCall {
        name: "crypto.hash.blake3".to_string(),
        thread: input[0].thread.clone(),
        logical_id: None,
        token: None,
        input: data.clone(),
        cause: None,
        correlation: input[0].correlation.clone(),
    }];
    *ctx.hpc_calls = Some(serde_cbor::to_vec(&out_hpc).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
