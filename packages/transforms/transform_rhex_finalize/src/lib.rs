use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        Rhex,
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::context::TransformContext,
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let (meta, mut rhex) = match &input[0].intent.data {
        RhexPayload::Mixed { meta, data } => {
            (meta, serde_cbor::from_slice::<Rhex>(&data[0]).unwrap())
        }
        _ => return -1,
    };

    let _ = rhex.calc_final_hash();

    let mut out_intent = RhexIntent::new(RhexIntent::gen_nonce());
    out_intent.schema = Binding::Bound("rhex://schema.rhex.store".to_string());
    out_intent.data = RhexPayload::Mixed {
        meta: meta.clone(),
        data: vec![serde_cbor::to_vec(&rhex).unwrap()],
    };

    transform_output.push(FluxItem {
        name: format!("rhex.store.{}", hex::encode(rhex.current_hash.unwrap())),
        thread: "rhex.store".to_string(),
        availability: FluxAvailability::Now,
        intent: out_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.rhex.store".to_string(),
            timestamp: 0,
        },
    });

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}
