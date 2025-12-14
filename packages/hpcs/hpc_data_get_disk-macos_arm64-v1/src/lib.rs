use std::io::Read;

use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta, payload::FluxPayload},
    hpc::{context::HpcContext, entry::HpcEntry},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DataReference {
    logical_id: Vec<u8>,
}

#[unsafe(no_mangle)]
pub extern "C" fn hpc_entry(ctx: *mut HpcContext) -> i32 {
    let ctx = unsafe { &mut *ctx };

    let data_ref = serde_cbor::from_slice::<DataReference>(ctx.input).unwrap();

    let filename = format!("/tmp/data/{}.rdat", hex::encode(&data_ref.logical_id));

    let mut buf = Vec::new();
    let mut file = match std::fs::File::open(&filename) {
        Ok(file) => file,
        Err(_) => return -1,
    };

    if file.read_to_end(&mut buf).is_err() {
        return -1;
    }

    // 1. Build typed flux locally
    let flux = vec![FluxItem {
        name: format!("data.get.result.{}", hex::encode(&data_ref.logical_id)),
        availability: FluxAvailability::Now,
        schema: None,
        payload: FluxPayload::Binary(buf),
        meta: FluxMeta {
            creator: "hpc.data.get.disk".to_string(),
            timestamp: 0,
        },
    }];

    // 2. Serialize once
    let bytes = serde_cbor::to_vec(&flux).unwrap();

    // 3. Assign the blob
    *ctx.output = Some(bytes);

    0
}

#[unsafe(no_mangle)]
pub static RHEX_HPC: HpcEntry = HpcEntry { entry: hpc_entry };
