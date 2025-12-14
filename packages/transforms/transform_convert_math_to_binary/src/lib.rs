use rhexis_core::{
    flux::{item::FluxItem, meta::FluxMeta, payload::FluxPayload},
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    // 1. Extract JSON payload
    let value = match &input[0].payload {
        FluxPayload::Json(v) => match v.get("value").and_then(|v| v.as_f64()) {
            Some(n) => n,
            None => return 1,
        },
        _ => return 1,
    };

    // 2. Encode f64 as canonical big-endian binary
    let bin_output = value.to_be_bytes().to_vec();

    let out_flux = vec![FluxItem {
        name: "console.write".to_string(),
        availability: rhexis_core::flux::availability::FluxAvailability::Now,
        schema: None,
        payload: FluxPayload::Binary(bin_output),
        meta: FluxMeta {
            creator: "transform.math.to.binary".to_string(),
            timestamp: 0,
        },
    }];

    *ctx.output = Some(serde_cbor::to_vec(&out_flux).unwrap());
    // 3. Emit Flux

    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
