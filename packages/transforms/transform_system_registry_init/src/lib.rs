use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::payload::RhexPayload,
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let logical_id = match &input[0].intent.data {
        RhexPayload::Binary { data } => data,
        _ => return -1,
    };

    transform_output.push(FluxItem {
        name: format!("data.get.{}", hex::encode(logical_id)),
        thread: "data.get".to_string(),
        availability: FluxAvailability::Now,
        intent: input[0].intent.clone(),
        correlation: None,
        meta: FluxMeta {
            creator: "transform.system.registry.init".to_string(),
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
