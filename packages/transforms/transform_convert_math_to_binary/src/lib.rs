use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{intent::RhexIntent, payload::RhexPayload},
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    // 1. Extract JSON payload
    let value = match &input[0].intent.data {
        RhexPayload::Json(v) => match v.get("value").and_then(|v| v.as_f64()) {
            Some(n) => n,
            None => return 1,
        },
        _ => return 1,
    };

    // 2. Encode f64 as canonical big-endian binary
    let bin_output = value.to_be_bytes().to_vec();

    let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
    intent.data = RhexPayload::Binary { data: bin_output };
    let out_flux = vec![FluxItem {
        name: "console.write".to_string(),
        thread: input[0].thread.clone(),
        availability: FluxAvailability::Now,
        intent,
        correlation: input[0].correlation.clone(),
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
