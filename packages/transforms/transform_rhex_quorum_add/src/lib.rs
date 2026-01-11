use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        Rhex,
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
        signature::{RhexSignature, SignatureType},
        states::RhexAwaitingQuorum,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let sig: RhexSignature = match &input[0].intent.data {
        RhexPayload::Binary { data } => serde_cbor::from_slice(&data).unwrap(),
        _ => return -1,
    };

    let (meta, data) = match &input[1].intent.data {
        RhexPayload::Mixed { meta, data } => (meta, data),
        _ => return -2,
    };

    let mut rhex: Rhex = serde_cbor::from_slice(&data[0]).unwrap();
    let meta: RhexAwaitingQuorum = serde_json::from_value(meta.clone()).unwrap();

    rhex.signatures.push(sig);
    let sig_count: usize = rhex
        .signatures
        .iter()
        .filter(|s| s.sig_type == SignatureType::Quorum)
        .count();

    // Do we have enough quorum signatures?
    if sig_count >= meta.count_needed {
        let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
        intent.schema = Binding::Bound("rhex://schema.rhex.quorum_met".to_string());
        intent.data = RhexPayload::Json(serde_json::json!({
            "scope": rhex.intent.scope,
            "quorum_count": sig_count,
            "quorum_required": meta.count_needed,
        }));

        transform_output.push(FluxItem {
            name: format!(
                "rhex.quorum_met.{}",
                hex::encode(rhex.current_hash.unwrap())
            ),
            thread: "rhex".to_string(),
            availability: FluxAvailability::Now,
            intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.rhex.analyze.quorum".to_string(),
                timestamp: 0,
            },
        });
    }

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
