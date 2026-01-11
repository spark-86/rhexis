use std::collections::HashMap;

use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use struct_registry::Registry;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let mut out_intent = RhexIntent::new(RhexIntent::gen_nonce());
    out_intent.schema = Binding::Bound("rhex://schema.system.registry".to_string());
    let registry: Registry = HashMap::new();

    out_intent.data = RhexPayload::Binary {
        data: serde_cbor::to_vec(&registry).unwrap(),
    };
    transform_output.push(FluxItem {
        name: "system.registry".to_string(),
        thread: "system.registry".to_string(),
        availability: FluxAvailability::Now,
        intent: out_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.system.registry.new".to_string(),
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
