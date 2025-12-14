use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta, payload::FluxPayload},
    membrane::HpcCall,
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DataReference {
    logical_id: Vec<u8>,
    data: Vec<u8>,
}

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(ctx.input).unwrap();
    if let FluxPayload::Mixed { meta: _, data } = &input[0].payload {
        let cause: i32 = 0x0001;
        let hex = hex::encode(data[0].clone());
        let data_ref = DataReference {
            logical_id: data[0].clone(),
            data: data[1].clone(),
        };

        let input = serde_cbor::to_vec(&data_ref).unwrap();

        let out_flux: Vec<FluxItem> = vec![FluxItem {
            name: format!("data.put.result.{}", hex),
            availability: FluxAvailability::Soon,
            schema: None,
            payload: FluxPayload::None,
            meta: FluxMeta {
                creator: "transform.data.put.disk".to_string(),
                timestamp: 0,
            },
        }];
        *ctx.output = Some(serde_cbor::to_vec(&out_flux).unwrap());

        let out_hpc: Vec<HpcCall> = vec![HpcCall {
            name: "data.put.disk".to_string(),
            logical_id: None,
            token: None,
            input,
            cause: Some(cause.to_be_bytes().to_vec()),
        }];

        *ctx.hpc_calls = Some(serde_cbor::to_vec(&out_hpc).unwrap());
    }
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
