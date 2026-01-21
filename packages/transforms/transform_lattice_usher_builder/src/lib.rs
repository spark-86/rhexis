use base64::engine::Engine as _;
use base64::engine::general_purpose;
use rhexis_core::flux::availability::FluxAvailability;
use rhexis_core::flux::meta::FluxMeta;
use rhexis_core::rhex::intent::Binding;
use rhexis_core::rhex::intent::RhexIntent;
use rhexis_core::{
    flux::item::FluxItem,
    rhex::payload::RhexPayload,
    transform::{context::TransformContext, entry::TransformEntry},
};
use struct_lattice::usher::Usher;
use struct_lattice::usher::UsherLocation;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let payload = match &input[0].intent.data {
        RhexPayload::Json(j) => j,
        _ => return -1,
    };

    let name = payload["name"].as_str().unwrap();
    let public_key_b64 = payload["public_key"].as_str().unwrap();
    let public_key: [u8; 32] = general_purpose::URL_SAFE_NO_PAD
        .decode(public_key_b64)
        .unwrap()
        .try_into()
        .unwrap();
    let priority = payload["priority"].as_u64().unwrap();
    let usher_loc = if payload["ip_addr"].as_str().is_some() && payload["port"].as_u64().is_some() {
        UsherLocation::Remote {
            ip_addr: payload["ip_addr"].as_str().unwrap().to_string(),
            port: payload["port"].as_u64().unwrap().try_into().unwrap(),
        }
    } else {
        UsherLocation::Local
    };

    let usher = Usher::new(name, public_key, priority, usher_loc);
    let mut usher_intent = RhexIntent::new(RhexIntent::gen_nonce());
    usher_intent.schema = Binding::Bound("rhex://schema.lattice.usher".to_string());
    usher_intent.data = RhexPayload::Binary {
        data: serde_cbor::to_vec(&usher).unwrap(),
    };

    transform_output.push(FluxItem {
        name: format!("lattice.usher.{}", hex::encode(&public_key)),
        thread: "lattice.ushers".to_string(),
        availability: FluxAvailability::Now,
        intent: usher_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.lattice.usher".to_string(),
            timestamp: 0,
        },
    });

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
