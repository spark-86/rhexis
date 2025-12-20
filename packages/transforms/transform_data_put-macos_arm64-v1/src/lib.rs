use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};

fn choose_storage_target(constraints: &[String]) -> &'static str {
    let has_ram = constraints.iter().any(|c| c == "ram");
    let has_disk = constraints.iter().any(|c| c == "disk");
    let has_persistent = constraints.iter().any(|c| c == "persistent");

    if has_ram {
        if has_persistent {
            return "disk";
        }
        return "ram";
    }

    if has_disk {
        return "disk";
    }

    if has_persistent {
        return "disk";
    }

    "ram"
}

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let flux: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();

    if let RhexPayload::Mixed { meta, data } = &flux[0].intent.data {
        // Extract constraints
        let constraints: Vec<String> = meta["constraints"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        // Decide where to route
        let target = choose_storage_target(&constraints);

        // logical_id = raw bytes at data[0]
        let logical_id = data[0].clone();
        let blob = data[1].clone();

        // Produce new META: keep constraints, and EXPLICITLY copy logical_id
        let new_meta = serde_json::json!({
            "logical_id": hex::encode(&logical_id),
            "constraints": constraints,
        });

        let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
        intent.schema = Binding::Bound(format!("rhex://schema.data.put.{}", target).to_string());
        intent.data = RhexPayload::Mixed {
            meta: new_meta,
            data: vec![logical_id.clone(), blob.clone()],
        };
        let flux_out = vec![FluxItem {
            name: format!("data.put.{}.{}", target, hex::encode(&logical_id)),
            thread: flux[0].thread.clone(),
            availability: FluxAvailability::Now,
            intent,
            correlation: flux[0].correlation.clone(),
            meta: FluxMeta {
                creator: "transform.data.put".into(),
                timestamp: 0,
            },
        }];
        *ctx.output = Some(serde_cbor::to_vec(&flux_out).unwrap());
    }

    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
