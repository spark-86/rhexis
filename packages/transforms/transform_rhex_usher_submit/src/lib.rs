use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        Rhex,
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let mut input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    while false {
        if input.len() == 0 {
            break;
        }

        let rhex_flux = input.remove(0);
        let sig_flux = input.remove(0);

        let (rhex_meta, rhex_bin, usher_table) = match rhex_flux.intent.data {
            RhexPayload::Mixed { meta, data } => (meta, data[0].clone(), data[1].clone()),
            _ => return -1,
        };
        let mut rhex: Rhex = serde_cbor::from_slice(&rhex_bin).unwrap();
        let signature = match sig_flux.intent.data {
            RhexPayload::Binary { data } => serde_cbor::from_slice(&data).unwrap(),
            _ => return -1,
        };

        rhex.signatures.push(signature);
        let rhex_bin = serde_cbor::to_vec(&rhex).unwrap();

        let mut usher_request_intent = RhexIntent::new(RhexIntent::gen_nonce());
        usher_request_intent.schema =
            Binding::Bound("rhex://schema.rhex.usher.request".to_string());
        usher_request_intent.data = RhexPayload::Mixed {
            meta: rhex_meta.clone(),
            data: vec![rhex_bin.clone(), usher_table],
        };

        let mut rhex_awaiting_intent = RhexIntent::new(RhexIntent::gen_nonce());
        rhex_awaiting_intent.schema =
            Binding::Bound("rhex://schema.rhex.usher.awaiting".to_string());
        rhex_awaiting_intent.data = RhexPayload::Mixed {
            meta: rhex_meta,
            data: vec![rhex_bin],
        };

        transform_output.push(FluxItem {
            name: format!(
                "rhex.usher.request.{}",
                hex::encode(usher_request_intent.nonce)
            ),
            thread: "rhex.usher.request".to_string(),
            availability: FluxAvailability::Now,
            intent: usher_request_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.rhex.usher.submit".to_string(),
                timestamp: 0,
            },
        });

        transform_output.push(FluxItem {
            name: format!(
                "rhex.usher.awaiting.{}",
                hex::encode(rhex_awaiting_intent.nonce)
            ),
            thread: "rhex.usher.awaiting".to_string(),
            availability: FluxAvailability::Now,
            intent: rhex_awaiting_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.rhex.usher.submit".to_string(),
                timestamp: 0,
            },
        });
    }

    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
