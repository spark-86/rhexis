use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    membrane::{CauseHeader, HpcCall},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct DataPayload {
    pub logical_id: [u8; 32],
    pub data: Vec<u8>,
}

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let payload_bytes = match &input[0].intent.data {
        RhexPayload::Binary { data } => data,
        _ => return -1,
    };
    let payloads: Vec<DataPayload> = serde_cbor::from_slice(&payload_bytes).unwrap();
    let mut out_hpc: Vec<HpcCall> = Vec::new();
    let mut out_flux: Vec<FluxItem> = Vec::new();

    for payload in payloads {
        let cause_payload = serde_cbor::to_vec(&RhexPayload::Json(json!({
            "action": "store",
            "logical_id": hex::encode(&payload.logical_id),
            "status": "complete",
        })))
        .unwrap();

        let cause = CauseHeader {
            target: format!("data.put.history.{}", hex::encode(&payload.logical_id)),
            thread: "data.put.history".to_string(),
            schema: "rhex://schema.data.put.history".to_string(),
            payload: cause_payload,
        };

        let hpc_call = HpcCall {
            name: "data.put.disk".to_string(),
            thread: "data.put".to_string(),
            logical_id: Some(payload.logical_id.to_vec()),
            token: None,
            input: serde_cbor::to_vec(&payload).unwrap(),
            cause: Some(serde_cbor::to_vec(&cause).unwrap()),
            correlation: None,
        };

        out_hpc.push(hpc_call);

        let mut flux_intent = RhexIntent::new(RhexIntent::gen_nonce());
        flux_intent.schema = Binding::Bound("rhex://schema.data.put.history".to_string());
        flux_intent.data = RhexPayload::Json(json!({
            "action": "store",
            "logical_id": hex::encode(&payload.logical_id),
            "status": "pending",
        }));

        let flux_item = FluxItem {
            name: format!("data.put.disk.{}", hex::encode(&payload.logical_id)),
            thread: "data.put".to_string(),
            availability: FluxAvailability::Soon,
            intent: flux_intent,
            correlation: input[0].correlation.clone(),
            meta: FluxMeta {
                creator: "transform.data.put.disk.fire".to_string(),
                timestamp: 0,
            },
        };
        out_flux.push(flux_item);
    }

    *ctx.output = Some(serde_cbor::to_vec(&out_flux).unwrap());
    *ctx.hpc_calls = Some(serde_cbor::to_vec(&out_hpc).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
