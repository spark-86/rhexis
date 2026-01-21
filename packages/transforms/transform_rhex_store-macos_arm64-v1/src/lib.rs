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
use struct_lattice::usher::{Usher, UsherLocation};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let (rhex_meta, rhex_data) = match &input[0].intent.data {
        RhexPayload::Mixed { meta, data } => (meta, data),
        _ => return -1,
    };

    let ushers: Vec<Usher> = serde_cbor::from_slice(&rhex_data[1]).unwrap();
    let target_usher = ushers.iter().find(|u| u.priority == 0).unwrap();

    let destination = match target_usher.location {
        UsherLocation::Local => "local",
        UsherLocation::Remote {
            ip_addr: _,
            port: _,
        } => "remote",
    };

    let rhex: Rhex = serde_cbor::from_slice(&rhex_data[0]).unwrap();
    let logical_id = rhex.current_hash.unwrap();
    let blob = rhex_data[1].clone();
    let scope = rhex_meta["scope"].as_str().unwrap().to_string();

    let mut store_intent = RhexIntent::new(RhexIntent::gen_nonce());
    let thread;

    if destination == "local" {
        thread = "data.put".to_string();
        store_intent.schema = Binding::Bound("rhex://schema.data.put".to_string());
        store_intent.data = RhexPayload::Mixed {
            meta: json!({
                "constraints": [
                    "disk",
                    destination,
                ],
            }),
            data: vec![logical_id.to_vec().clone(), blob.clone()],
        };
    } else {
        thread = "lattice.put".to_string();
        store_intent.schema = Binding::Bound("rhex://schema.lattice.put".to_string());
        store_intent.data = RhexPayload::Mixed {
            meta: json!({
            "action": "append",
            "scope": scope,
            }),
            data: vec![
                logical_id.to_vec().clone(),
                serde_cbor::to_vec(&rhex).unwrap(),
            ],
        };
    }

    let mut commit_intent = RhexIntent::new(RhexIntent::gen_nonce());
    commit_intent.schema = Binding::Bound("rhex://schema.rhex.commit".to_string());
    commit_intent.data = RhexPayload::Json(rhex_meta.clone());

    let out_flux = vec![
        FluxItem {
            name: format!("{}.{}", &thread, hex::encode(&logical_id)),
            thread: thread.clone(),
            availability: FluxAvailability::Now,
            intent: store_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.rhex.store".to_string(),
                timestamp: 0,
            },
        },
        FluxItem {
            name: format!("rhex.commit.{}", hex::encode(&logical_id)),
            thread: "rhex".to_string(),
            availability: FluxAvailability::Soon,
            intent: commit_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.rhex.store".to_string(),
                timestamp: 0,
            },
        },
    ];

    *ctx.output = Some(serde_cbor::to_vec(&out_flux).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
