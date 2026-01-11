use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        Rhex,
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde_json::json;
use struct_lattice::usher::Usher;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();
    let (meta, rhex) = match &input[0].intent.data {
        RhexPayload::Mixed { meta, data } => (
            meta.clone(),
            serde_cbor::from_slice::<Rhex>(&data[0]).unwrap(),
        ),
        _ => return -1,
    };

    let all_ushers: Vec<Usher> =
        serde_json::from_value(meta.get("ushers").unwrap().clone()).unwrap();

    let unsigned_ushers = all_ushers
        .iter()
        .filter(|u| !rhex.signatures.iter().any(|s| s.public_key == u.public_key))
        .collect::<Vec<_>>();

    let mut name: String;
    let mut thread: String;

    for usher in unsigned_ushers {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&rhex.current_hash.unwrap());
        hasher.update(&usher.public_key);
        let hash = hasher.finalize();

        let mut usher_intent = RhexIntent::new(RhexIntent::gen_nonce());

        if usher.ip_address.is_some() {
            let mut cloned_meta = meta.clone();
            name = format!(
                "lattice.quorum.remote.signature.request.{}",
                hex::encode(hash.as_bytes())
            );
            thread = "lattice.quorum.remote.signature.requests".to_string();
            cloned_meta["target"] = json!({
                "location": "remote",
                "name": usher.name,
                "public_key": hex::encode(usher.public_key),
                "priority": usher.priority,
                "ip_address": usher.ip_address.clone(),
                "port": usher.port.unwrap(),
            });
            usher_intent.schema =
                Binding::Bound("rhex://schema.lattice.quorum.signature.request".to_string());
            usher_intent.data = RhexPayload::Mixed {
                meta: cloned_meta,
                data: vec![serde_cbor::to_vec(&rhex).unwrap()],
            }
        } else {
            let mut cloned_meta = meta.clone();
            name = format!(
                "lattice.quorum.local.signature.request.{}",
                hex::encode(hash.as_bytes())
            );
            thread = "lattice.quorum.local.signature.requests".to_string();
            cloned_meta["target"] = json!({
                "location": "local",
                "name": usher.name,
                "public_key": hex::encode(usher.public_key),
                "priority": usher.priority,
            });
            usher_intent.schema =
                Binding::Bound("rhex://schema.lattice.quorum.local.signature".to_string());
            usher_intent.data = RhexPayload::Mixed {
                meta: meta.clone(),
                data: vec![serde_cbor::to_vec(&rhex).unwrap()],
            }
        }

        transform_output.push(FluxItem {
            name,
            thread,
            availability: FluxAvailability::Now,
            intent: usher_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.lattice.quorum.request.signatures".to_string(),
                timestamp: 0,
            },
        })
    }
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
