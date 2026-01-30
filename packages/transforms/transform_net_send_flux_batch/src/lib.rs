use std::collections::HashMap;

use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde_json::json;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let mut batch_map: HashMap<(String, u16), Vec<FluxItem>> = HashMap::new();

    for flux in input {
        let (meta, data) = match flux.intent.data {
            RhexPayload::Mixed { meta, data } => (meta, data),
            _ => return -1,
        };

        let ip_addr = meta["ip_addr"].as_str();
        if ip_addr.is_none() {
            return -2;
        }
        let ip_addr = ip_addr.unwrap();
        let port = meta["port"].as_u64();
        if port.is_none() {
            return -3;
        }
        let port = port.unwrap();

        if batch_map.contains_key(&(ip_addr.to_string(), port.try_into().unwrap())) {
            batch_map
                .get_mut(&(ip_addr.to_string(), port.try_into().unwrap()))
                .unwrap()
                .push(serde_cbor::from_slice(&data[0]).unwrap());
        } else {
            batch_map.insert(
                (ip_addr.to_string(), port.try_into().unwrap()),
                vec![serde_cbor::from_slice(&data[0]).unwrap()],
            );
        }
    }

    for (key, value) in batch_map {
        let mut batch_intent = RhexIntent::new(RhexIntent::gen_nonce());
        let mut data: Vec<Vec<u8>> = Vec::new();
        for flux in value {
            data.push(serde_cbor::to_vec(&flux).unwrap());
        }
        batch_intent.schema = Binding::Bound("rhex://schema.net.send.flux.batch".to_string());
        batch_intent.data = RhexPayload::Mixed {
            meta: json!({
                "ip_addr": key.0,
                "port": key.1,
            }),
            data,
        };

        transform_output.push(FluxItem {
            name: format!("net.send.flux.batch.{}.{}", key.0, key.1),
            thread: "net.send.flux.batches".to_string(),
            availability: FluxAvailability::Now,
            intent: batch_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.net.send.flux.batch".to_string(),
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
