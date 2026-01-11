use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    transform_output.push(FluxItem {
        name: "system.registry".to_string(),
        thread: "system.registry".to_string(),
        availability: FluxAvailability::Now,
        intent: input[0].intent.clone(),
        correlation: None,
        meta: FluxMeta {
            creator: "transform.system.registry.load".to_string(),
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
