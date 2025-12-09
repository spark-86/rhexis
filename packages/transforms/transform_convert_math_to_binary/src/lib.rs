use rhexis_core::{
    flux::{item::FluxItem, meta::FluxMeta, payload::FluxPayload},
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };

    // 1. Extract JSON payload
    let value = match &ctx.input[0].payload {
        FluxPayload::Json(v) => match v.get("value").and_then(|v| v.as_f64()) {
            Some(n) => n,
            None => return 1,
        },
        _ => return 1,
    };

    // 2. Encode f64 as canonical big-endian binary
    let bin_output = value.to_be_bytes().to_vec();

    // 3. Emit Flux
    ctx.output.push(FluxItem {
        name: "console.write".to_string(),
        schema: None,
        payload: FluxPayload::Binary(bin_output),
        meta: FluxMeta {
            creator: "transform.math.to.binary".to_string(),
            timestamp: 0,
        },
    });

    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
