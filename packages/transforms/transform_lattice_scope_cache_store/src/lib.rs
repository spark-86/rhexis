use rand::random;
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
    let mut input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let table_flux = input.remove(0);
    let registry_flux = input.remove(0);
    let table_data = match table_flux.intent.data {
        RhexPayload::Binary { data } => data,
        _ => return -1,
    };
    let registry_data = match registry_flux.intent.data {
        RhexPayload::Binary { data } => data,
        _ => return -1,
    };

    let registry: Registry = serde_cbor::from_slice(&registry_data).unwrap();

    let mut store_logical_id = false;
    let logical_id: [u8; 32] = if registry.contains_key("lattice.scope.cache") {
        let id = registry.get("lattice.scope.cache").unwrap();
        match id {
            RegistryEntry::LogicalId(i) => i.clone(),
            _ => return -1,
        }
    } else {
        store_logical_id = true;
        random()
    };

    let mut store_intent = RhexIntent::new(RhexIntent::gen_nonce());
    store_intent.schema = Binding::Bound("rhex://schema.data.put".to_string());
    store_intent.data = RhexPayload::Mixed {
        meta: json!({
            "constraints": ["local", "permanent"]
        }),
        data: vec![logical_id.to_vec(), table_data],
    };
    transform_output.push(FluxItem {
        name: format!("data.put.{}", hex::encode(&logical_id)),
        thread: "data.put".to_string(),
        availability: FluxAvailability::Now,
        intent: store_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.lattice.scope.cache.store".to_string(),
            timestamp: 0,
        },
    });

    if store_logical_id {
        let mut put_intent = RhexIntent::new(RhexIntent::gen_nonce());
        put_intent.schema = Binding::Bound("rhex://schema.system.registry.add".to_string());
        put_intent.data = RhexPayload::Mixed {
            meta: json!({
                "action": "add",
                "data_type": "logical_id",
            }),
            data: vec![logical_id.to_vec()],
        };
        transform_output.push(FluxItem {
            name: format!("system.registry.add.{}", hex::encode(&logical_id)),
            thread: "system.registry.add".to_string(),
            availability: FluxAvailability::Now,
            intent: put_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.lattice.scope.cache.store".to_string(),
                timestamp: 0,
            },
        });
    }

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
