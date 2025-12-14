use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta, payload::FluxPayload},
    membrane::{ActionType, MembraneAction},
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde_json::json;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };

    // ---- decode input flux ----
    let input: Vec<FluxItem> = match serde_cbor::from_slice(ctx.input) {
        Ok(v) => v,
        Err(_) => return -1,
    };

    if input.is_empty() {
        return 0;
    }

    let bucket = &input[0];

    let payload_bytes = match &bucket.payload {
        FluxPayload::Binary(b) => b,
        _ => return -1,
    };

    let actions: Vec<MembraneAction> = match serde_cbor::from_slice(payload_bytes) {
        Ok(v) => v,
        Err(_) => return -1,
    };

    // ---- build output flux locally ----
    let mut flux_out: Vec<FluxItem> = Vec::new();

    for action in actions {
        let cause = match action.cause {
            Some(c) => c,
            None => continue,
        };

        // Simple, deterministic fan-out
        let (prefix, schema) = match action.action {
            _ => (
                "system.membrane.event".to_string(),
                "rhex://schema.system.membrane.event".to_string(),
            ),
        };

        let name = format!("{}.{}", prefix, hex::encode(&action.logical_id));
        let meta = match action.action {
            ActionType::RegisterResource => json!({
                "register": ""
            }),
            ActionType::ReleaseResource => json!({
                "release": ""
            }),
            ActionType::IoComplete => json!({
                "io_complete": ""
            }),
        };
        flux_out.push(FluxItem {
            name,
            availability: FluxAvailability::Now,
            schema: Some(schema),
            payload: FluxPayload::Mixed {
                meta,
                data: vec![cause],
            },
            meta: FluxMeta {
                creator: "transform.membrane.fanout".to_string(),
                timestamp: 0,
            },
        });
    }

    // ---- serialize once, assign once ----
    if !flux_out.is_empty() {
        let bytes = match serde_cbor::to_vec(&flux_out) {
            Ok(b) => b,
            Err(_) => return -1,
        };
        *ctx.output = Some(bytes);
    }

    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
