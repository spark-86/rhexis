use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde_json::json;
use struct_registry::{Registry, RegistryEntry};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let registry_bin = match &input[1].intent.data {
        RhexPayload::Binary { data } => data,
        _ => return -1,
    };

    let registry: Registry = serde_cbor::from_slice(&registry_bin).unwrap();

    println!("{:?}", registry);

    let registry_id = registry.get("registry.logical_id");
    if registry_id.is_none() {
        return -2;
    }
    let registry_id = match registry_id.unwrap() {
        RegistryEntry::LogicalId(id) => id,
        _ => return -3,
    };

    let mut put_intent = RhexIntent::new(RhexIntent::gen_nonce());
    put_intent.schema = Binding::Bound("rhex://schema.data.put".to_string());
    put_intent.data = RhexPayload::Mixed {
        meta: json!({
            "constraints": ["local","permanent"]
        }),
        data: vec![registry_id.to_vec(), registry_bin.to_vec()],
    };
    transform_output.push(FluxItem {
        name: format!("data.put.{}", hex::encode(registry_id)),
        thread: "data.put".to_string(),
        availability: FluxAvailability::Now,
        intent: put_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.system.registry.store".to_string(),
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
