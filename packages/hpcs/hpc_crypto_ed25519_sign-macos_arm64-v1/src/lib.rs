use ed25519_dalek::Signer;
use rand::random;
use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    hpc::{context::HpcContext, entry::HpcEntry, envelope::HpcCallEnvelope},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
};
use std::fs::read;

#[unsafe(no_mangle)]
pub extern "C" fn hpc_entry(ctx: *mut HpcContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: HpcCallEnvelope = serde_cbor::from_slice(ctx.input).unwrap();

    let payload: Vec<Vec<u8>> = serde_cbor::from_slice(&input.payload).unwrap();
    let public_key: [u8; 32] = payload[0].as_slice().try_into().unwrap();
    let hash: [u8; 32] = payload[1].as_slice().try_into().unwrap();

    let filename = format!("/tmp/data/keys/{}.key", hex::encode(public_key.clone()));
    let keyfile = read(filename).unwrap();
    let keyfile = keyfile
        .as_slice()
        .try_into()
        .expect("key file must be exactly 32 bytes");
    let key = ed25519_dalek::SigningKey::from_bytes(keyfile);

    let signature = key.sign(&hash);

    let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
    intent.schema = Binding::Bound("rhex://schema.crypto.ed25519.signed".to_string());
    intent.data = RhexPayload::Binary {
        data: signature.to_vec(),
    };

    let id: [u8; 32] = random();
    let out_flux = vec![FluxItem {
        name: format!("crypto.ed25519.signed.{}", hex::encode(&id)),
        thread: input.thread.clone(),
        availability: FluxAvailability::Now,
        intent,
        correlation: None,
        meta: FluxMeta {
            creator: "hpc.crypto.ed25519.sign".to_string(),
            timestamp: 0,
        },
    }];

    *ctx.output = Some(serde_cbor::to_vec(&out_flux).unwrap());

    0
}

#[unsafe(no_mangle)]
pub static RHEX_HPC: HpcEntry = HpcEntry { entry: hpc_entry };
