use std::collections::HashMap;

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
use struct_lattice::LatticeScopeLocation;
use struct_registry::RegistryEntry;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let logical_id: [u8; 32] = random();
    let lattice_scope_table: HashMap<String, Vec<LatticeScopeLocation>> = HashMap::new();

    let mut cache_table_intent = RhexIntent::new(RhexIntent::gen_nonce());
    cache_table_intent.schema =
        Binding::Bound("rhex://schema.lattice.scope.cache.table".to_string());
    cache_table_intent.data = RhexPayload::Binary {
        data: serde_cbor::to_vec(&lattice_scope_table).unwrap(),
    };
    transform_output.push(FluxItem {
        name: "lattice.scope.cache.table".to_string(),
        thread: "lattice.scope.cache.table".to_string(),
        availability: FluxAvailability::Now,
        intent: cache_table_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.lattice.scope.cache.create".to_string(),
            timestamp: 0,
        },
    });

    let mut data_put_intent = RhexIntent::new(RhexIntent::gen_nonce());
    data_put_intent.schema = Binding::Bound("rhex://schema.data.put".to_string());
    data_put_intent.data = RhexPayload::Mixed {
        meta: json!({
            "constraints": ["local", "permanent"]
        }),
        data: vec![
            logical_id.to_vec(),
            serde_cbor::to_vec(&lattice_scope_table).unwrap(),
        ],
    };
    transform_output.push(FluxItem {
        name: format!("data.put.{}", hex::encode(&logical_id)),
        thread: "data.put".to_string(),
        availability: FluxAvailability::Now,
        intent: data_put_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.lattice.scope.cache.create".to_string(),
            timestamp: 0,
        },
    });

    let mut registry_add_intent = RhexIntent::new(RhexIntent::gen_nonce());
    registry_add_intent.schema = Binding::Bound("rhex://schema.system.registry.add".to_string());
    registry_add_intent.data = RhexPayload::Mixed {
        meta: json!({
            "action": "add",
            "data_type": "logical_id"
        }),
        data: vec![serde_cbor::to_vec(&RegistryEntry::LogicalId(logical_id)).unwrap()],
    };
    transform_output.push(FluxItem {
        name: format!("system.registry.add.{}", hex::encode(&logical_id)),
        thread: "system.registry.add".to_string(),
        availability: FluxAvailability::Now,
        intent: registry_add_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.lattice.scope.cache.create".to_string(),
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
