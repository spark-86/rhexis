use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde::Serialize;

#[derive(Serialize)]
struct DataPayload {
    pub logical_id: [u8; 32],
    pub data: Vec<u8>,
}

enum Location {
    Disk,
    Ram,
    Network,
    Unknown,
}

fn sort_location(constraints: &Vec<String>) -> Location {
    let is_permanent = constraints.contains(&"permanent".to_string());
    let _is_temporary = constraints.contains(&"temporary".to_string());
    let is_local = constraints.contains(&"local".to_string());
    let is_remote = constraints.contains(&"remote".to_string());
    let is_disk = constraints.contains(&"disk".to_string());
    let is_ram = constraints.contains(&"ram".to_string());
    let is_network = constraints.contains(&"network".to_string());

    if is_local {
        if is_permanent {
            return Location::Disk;
        } else {
            return Location::Ram;
        }
    }
    if is_remote {
        return Location::Network;
    }
    if is_disk {
        return Location::Disk;
    }
    if is_ram {
        return Location::Ram;
    }
    if is_network {
        return Location::Network;
    }
    Location::Unknown
}

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    for item in input {
        let (meta, logical_id, data) = match &item.intent.data {
            RhexPayload::Mixed { meta, data } => (
                meta.clone(),
                data[0].clone().try_into().unwrap(),
                data[1].clone(),
            ),
            _ => return -1,
        };

        let constraints = meta
            .get("constraints")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f.as_str().unwrap().to_string())
            .collect();
        let location = sort_location(&constraints);
        let loc_string = match location {
            Location::Disk => "disk",
            Location::Ram => "ram",
            Location::Network => "network",
            Location::Unknown => "unknown",
        };

        let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
        intent.schema = Binding::Bound(format!("rhex://schema.data.put.{}", &loc_string));

        intent.data = match location {
            Location::Disk | Location::Ram => RhexPayload::Binary {
                data: serde_cbor::to_vec(&DataPayload { logical_id, data }).unwrap(),
            },
            _ => return -2,
        };

        transform_output.push(FluxItem {
            name: format!("data.put.{}", loc_string),
            thread: "data.put".to_string(),
            availability: FluxAvailability::Now,
            intent,
            correlation: item.correlation.clone(),
            meta: FluxMeta {
                creator: "transform.data.put".to_string(),
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
