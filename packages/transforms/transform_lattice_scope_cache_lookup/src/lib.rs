use std::collections::HashMap;

use rand::seq::IndexedRandom;
use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde_json::json;
use struct_lattice::{LatticeScopeLocation, usher::UsherLocation};

fn find_closest_parent(
    target_scope: String,
    table: &HashMap<String, LatticeScopeLocation>,
) -> String {
    let mut working = target_scope;

    loop {
        if table.contains_key(&working) {
            return working;
        }

        if let Some(idx) = working.rfind('.') {
            working.truncate(idx);
        } else {
            // No more dots; fall back to root scope
            return "".to_string();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();
    let mut requests: Vec<FluxItem> = Vec::new();

    let mut table_flux = FluxItem::new();
    let mut ip_addr_flux = FluxItem::new();

    // Parse inputs
    for item in input {
        match item.intent.schema {
            Binding::Bound(ref s) => match s.as_str() {
                "rhex://schema.lattice.scope.cache.request" => requests.push(item.clone()),
                "rhex://schema.lattice.scope.cache.table" => table_flux = item,
                "rhex://schema.system.ip_address" => ip_addr_flux = item,
                _ => {
                    println!("Unknown schema: {}", s);
                    return -1;
                }
            },
            _ => return -2,
        }
    }

    // Build table
    let table: HashMap<String, LatticeScopeLocation> = match table_flux.intent.data {
        RhexPayload::Binary { data } => serde_cbor::from_slice(&data).unwrap(),
        _ => return -3,
    };

    // Get IP address
    let (ip_addr, port) = match ip_addr_flux.intent.data {
        RhexPayload::Json(ref v) => {
            let ip_addr = v["ip_addr"].as_str().unwrap();
            let port = v["port"].as_u64().unwrap();
            (ip_addr, port)
        }
        _ => return -4,
    };

    // Process requests
    for request in requests {
        let local_entry = table.get(&request.name);
        let scope = match request.intent.data {
            RhexPayload::Json(ref v) => v["scope"].as_str().unwrap(),
            _ => return -8,
        };
        let mut out_intent: RhexIntent = RhexIntent::new(RhexIntent::gen_nonce());

        // Entry found in the cache. Determine local/remote
        if local_entry.is_some() {
            let entry = local_entry.unwrap();
            match entry {
                LatticeScopeLocation::Local(location) => {
                    out_intent.data = RhexPayload::Mixed {
                        meta: json!({
                            "distance": "local",
                            "scope": scope,
                            "status": "complete",
                        }),
                        data: vec![serde_cbor::to_vec(&location).unwrap()],
                    }
                }
                LatticeScopeLocation::Remote(location) => {
                    out_intent.data = RhexPayload::Mixed {
                        meta: json!({
                            "distance": "remote",
                            "scope": scope,
                            "status": "complete",
                        }),
                        data: vec![serde_cbor::to_vec(&location).unwrap()],
                    }
                }
                LatticeScopeLocation::Unknown => return -7,
            };

            out_intent.schema =
                Binding::Bound("rhex://schema.lattice.scope.lookup.result".to_string());
            transform_output.push(FluxItem {
                name: format!("lattice.scope.lookup.result.{}", hex::encode(&request.name)),
                thread: "lattice.scope.lookup.result".to_string(),
                availability: request.availability,
                intent: out_intent,
                correlation: request.correlation,
                meta: FluxMeta {
                    creator: "transform.lattice.scope.lookup".to_string(),
                    timestamp: 0,
                },
            });
        } else {
            // Location is not known to us, build the remote request
            let closest_parent = find_closest_parent(scope.to_string(), &table);

            let parent_loc = table.get(&closest_parent).unwrap();
            let contact = match parent_loc {
                LatticeScopeLocation::Local(location) => {
                    // I'm kinda fucking iffy on this. Like we should never be
                    // here, right? because if we host this parent locally it
                    // SHOULD know it's children.
                    location.ushers.clone()
                }
                LatticeScopeLocation::Remote(location) => location.ushers.clone(),
                LatticeScopeLocation::Unknown => return -5,
            };

            // Should this emit another flux and have that resolve
            // usher weight? Yup. Are we gonna do that now instead
            // of overloading this transform? Nope.
            let random_contact = contact.choose(&mut rand::rng()).unwrap();
            let (remote_ip, remote_port) = match &random_contact.location {
                UsherLocation::Remote { ip_addr, port } => (ip_addr, port),
                _ => return -6,
            };

            let mut remote_intent = RhexIntent::new(RhexIntent::gen_nonce());
            remote_intent.schema =
                Binding::Bound("rhex://schema.lattice.scope.remote.lookup.request".to_string());
            remote_intent.data = RhexPayload::Json(json!({
                "return": {
                    "ip_addr": ip_addr,
                    "port": port,
                },
                "scope": scope,
            }));

            let remote_flux = FluxItem {
                name: scope.to_string(),
                thread: "lattice.scope.remote.lookup.request".to_string(),
                availability: FluxAvailability::Now,
                intent: remote_intent,
                correlation: None,
                meta: FluxMeta {
                    creator: "transform.lattice.scope.cache.lookup".to_string(),
                    timestamp: 0,
                },
            };
            let remote_bin = serde_cbor::to_vec(&remote_flux).unwrap();
            let mut net_flux = RhexIntent::new(RhexIntent::gen_nonce());
            net_flux.schema = Binding::Bound("rhex://schema.net.send.flux".to_string());
            net_flux.data = RhexPayload::Mixed {
                meta: json!({
                    "ip_addr": remote_ip,
                    "port": remote_port
                }),
                data: vec![remote_bin],
            };

            transform_output.push(FluxItem {
                name: scope.to_string(),
                thread: "net.send.flux".to_string(),
                availability: request.availability,
                intent: net_flux,
                correlation: None,
                meta: FluxMeta {
                    creator: "transform.lattice.scope.cache.lookup".to_string(),
                    timestamp: 0,
                },
            });
        }
    }

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
