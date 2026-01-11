use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::intent::{Binding, RhexIntent},
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let mut out_intent = RhexIntent::new(RhexIntent::gen_nonce());
    out_intent.schema = Binding::Bound("rhex://schema.lattice.scope.cache.table".to_string());
    out_intent.data = input[0].intent.data.clone();

    transform_output.push(FluxItem {
        name: "lattice.scope.cache.table".to_string(),
        thread: "lattice.scope.cache.table".to_string(),
        availability: FluxAvailability::Now,
        intent: out_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.lattice.scope.cache.load".to_string(),
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
