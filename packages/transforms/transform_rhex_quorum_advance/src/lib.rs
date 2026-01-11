use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        Rhex,
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
        signature::{RhexSignature, SignatureType},
    },
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let signature: RhexSignature = match &input[0].intent.data {
        RhexPayload::Mixed { meta: _, data } => serde_cbor::from_slice(&data[0]).unwrap(),
        _ => return -1,
    };

    let (meta, mut rhex) = match &input[1].intent.data {
        RhexPayload::Mixed { meta, data } => {
            (meta, serde_cbor::from_slice::<Rhex>(&data[0]).unwrap())
        }
        _ => return -1,
    };

    rhex.signatures.push(signature);

    let quorum_required = meta.get("quorum_k").unwrap().as_u64().unwrap() as usize;
    let quorum_count = rhex
        .signatures
        .iter()
        .filter(|f| f.sig_type == SignatureType::Quorum)
        .count();

    let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
    let name: String;
    let thread: String;
    if quorum_count >= quorum_required {
        intent.schema = Binding::Bound("rhex://schema.rhex.finalize".to_string());
        name = format!("rhex.finalize.{}", hex::encode(rhex.current_hash.unwrap()));
        thread = "rhex.finalize".to_string();
    } else {
        intent.schema = Binding::Bound("rhex://schema.rhex.awaiting_quorum".to_string());
        name = format!(
            "rhex.awaiting_quorum.{}",
            hex::encode(rhex.current_hash.unwrap())
        );
        thread = "rhex.quorum".to_string();
    };
    intent.data = RhexPayload::Mixed {
        meta: meta.clone(),
        data: vec![serde_cbor::to_vec(&rhex).unwrap()],
    };
    transform_output.push(FluxItem {
        name,
        thread,
        availability: FluxAvailability::Now,
        intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.rhex.quorum.advance".to_string(),
            timestamp: 0,
        },
    });

    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
