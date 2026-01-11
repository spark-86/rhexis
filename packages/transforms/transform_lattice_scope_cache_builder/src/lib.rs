use std::collections::HashMap;

use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::context::TransformContext,
};
use struct_lattice::{
    LatticeScopeCacheAction, LatticeScopeCacheAdd, LatticeScopeCacheRemove,
    LatticeScopeCacheUpdate, LatticeScopeLocation,
};

fn add_location(action: LatticeScopeCacheAdd, table: &mut HashMap<String, LatticeScopeLocation>) {
    let location = action.location.clone();
    let scope = match &action.location {
        LatticeScopeLocation::Local(l) => l.scope.clone(),
        LatticeScopeLocation::Remote(r) => r.scope.clone(),
        _ => return,
    };
    table.insert(scope, location);
}

fn update_location(
    action: LatticeScopeCacheUpdate,
    table: &mut HashMap<String, LatticeScopeLocation>,
) {
    let location = action.location.clone();
    let scope = match &action.location {
        LatticeScopeLocation::Local(l) => l.scope.clone(),
        LatticeScopeLocation::Remote(r) => r.scope.clone(),
        _ => return,
    };
    table.remove(&scope);
    table.insert(scope, location);
}

fn remove_location(
    action: LatticeScopeCacheRemove,
    table: &mut HashMap<String, LatticeScopeLocation>,
) {
    let scope = action.scope.clone();
    table.remove(&scope);
}

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();
    let mut actions: Vec<LatticeScopeCacheAction> = Vec::new();
    let mut table: HashMap<String, LatticeScopeLocation> = HashMap::new();

    for item in input {
        if item.intent.schema
            == Binding::Bound("rhex://schema.lattice.scope.cache.action".to_string())
        {
            let data = match item.intent.data {
                RhexPayload::Json(j) => j,
                _ => return -1,
            };
            actions.push(serde_json::from_value(data).unwrap());
        } else {
            table = match item.intent.data {
                RhexPayload::Binary { data } => serde_cbor::from_slice(&data).unwrap(),
                _ => return -2,
            }
        }
    }

    for item in actions {
        match item {
            LatticeScopeCacheAction::Add(a) => add_location(a, &mut table),
            LatticeScopeCacheAction::Update(u) => update_location(u, &mut table),
            LatticeScopeCacheAction::Remove(r) => remove_location(r, &mut table),
        };
    }

    let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
    intent.schema = Binding::Bound("rhex://schema.lattice.scope.cache.table".to_string());
    intent.data = RhexPayload::Binary {
        data: serde_cbor::to_vec(&table).unwrap(),
    };

    transform_output.push(FluxItem {
        name: "lattice.scope.cache.table".to_string(),
        thread: "lattice.scope.cache".to_string(),
        availability: FluxAvailability::Now,
        intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.lattice.scope.cache.builder".to_string(),
            timestamp: 0,
        },
    });

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}
