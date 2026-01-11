use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let init_item = input[0].clone();

    let init_logical_id = match init_item.intent.data {
        RhexPayload::Mixed { meta: _, data } => data[0].clone(),
        _ => return -1,
    };

    // data.get
    let mut get_intent = RhexIntent::new(RhexIntent::gen_nonce());
    get_intent.schema = Binding::Bound("rhex://schema.data.get".to_string());
    get_intent.data = RhexPayload::Binary {
        data: init_logical_id.to_vec(),
    };

    transform_output.push(FluxItem {
        name: format!("data.get.{}", hex::encode(init_logical_id)),
        thread: "data.get".to_string(),
        availability: FluxAvailability::Now,
        intent: get_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.lattice.initializer".to_string(),
            timestamp: 0,
        },
    });

    // lattice.scope.cache.table
    let mut cache_intent = RhexIntent::new(RhexIntent::gen_nonce());
    cache_intent.schema = Binding::Bound("rhex://schema.lattice.scope.cache.table".to_string());
    cache_intent.data = RhexPayload::None;

    transform_output.push(FluxItem {
        name: "lattice.scope.cache.table".to_string(),
        thread: "lattice.scope.cache.table".to_string(),
        availability: FluxAvailability::Soon,
        intent: cache_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.lattice.initializer".to_string(),
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
