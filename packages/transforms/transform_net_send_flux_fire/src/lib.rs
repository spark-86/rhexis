use rhexis_core::{
    flux::item::FluxItem,
    membrane::HpcCall,
    rhex::payload::RhexPayload,
    transform::{context::TransformContext, entry::TransformEntry},
};
use struct_net::NetFlux;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(ctx.input).unwrap();

    let mut hpc_calls: Vec<HpcCall> = Vec::new();

    for flux in input {
        let (meta, data) = match flux.intent.data {
            RhexPayload::Mixed { meta, data } => (meta, data),
            _ => return -1,
        };

        let ip_addr = meta["ip_addr"].as_str().unwrap();
        let port = meta["port"].as_u64().unwrap() as u16;
        let mut payload: Vec<FluxItem> = Vec::new();
        for d in data {
            payload.push(serde_cbor::from_slice(&d).unwrap());
        }

        hpc_calls.push(HpcCall {
            name: "net.send.flux".to_string(),
            logical_id: None,
            thread: flux.thread.clone(),
            token: None,
            input: serde_cbor::to_vec(&NetFlux {
                sig: None,
                key: None,
                ip_addr: ip_addr.to_string(),
                port,
                payload,
                gt: 0,
            })
            .unwrap(),
            cause: None,
            correlation: flux.correlation.clone(),
        });
    }

    *ctx.hpc_calls = Some(serde_cbor::to_vec(&hpc_calls).unwrap());

    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
