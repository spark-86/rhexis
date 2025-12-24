use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DataPayload {
    pub logical_id: [u8; 32],
    pub data: Vec<u8>,
}

const FIRES_PER_CYCLE: usize = 8;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();
    let mut count = 0;
    let mut out_batches: Vec<Vec<DataPayload>> = Vec::new();
    let mut batch: Vec<DataPayload> = Vec::new();
    for item in input {
        let payload_bytes = match &item.intent.data {
            RhexPayload::Binary { data } => {
                serde_cbor::from_slice::<DataPayload>(data.as_slice()).unwrap()
            }
            _ => return -1,
        };
        batch.push(payload_bytes);
        count += 1;
        if count == FIRES_PER_CYCLE {
            out_batches.push(batch.clone());
            batch.clear();
            count = 0;
        }
    }
    if !batch.is_empty() {
        out_batches.push(batch);
    }
    for batch in out_batches {
        let output = serde_cbor::to_vec(&batch).unwrap();
        let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
        intent.schema = Binding::Bound("rhex://schema.data.put.disk.batch".to_string());
        intent.data = RhexPayload::Binary {
            data: output.clone(),
        };
        let id = hex::encode(blake3::hash(&output).as_bytes().to_vec());

        let out_flux = FluxItem {
            name: format!("data.put.disk.batch.{}", id),
            thread: "data.put.disk.queue".to_string(),
            availability: FluxAvailability::Now,
            intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.data.put.disk.batch".to_string(),
                timestamp: 0,
            },
        };

        transform_output.push(out_flux);
    }

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
