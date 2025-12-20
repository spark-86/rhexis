use std::fs::write;

use ed25519_dalek::SigningKey;

use rand::random;
use rand_core::OsRng;
use rhexis_core::hpc::{
    context::HpcContext,
    directive::{MembraneDirective, ResourceBacking},
    entry::HpcEntry,
    envelope::HpcCallEnvelope,
};

fn gen_random() -> [u8; 32] {
    random()
}

#[unsafe(no_mangle)]
pub extern "C" fn hpc_entry(ctx: *mut HpcContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: HpcCallEnvelope = serde_cbor::from_slice(ctx.input).unwrap();
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    let key_filename = format!(
        "/tmp/data/keys/{}.key",
        hex::encode(&verifying_key.as_bytes())
    );
    let _ = write(key_filename, signing_key.as_bytes());

    let out_membrane = vec![MembraneDirective::RegisterResource {
        logical_id: verifying_key.as_bytes().to_vec(),
        token: gen_random().to_vec(),
        backing: ResourceBacking {
            kind: "crypto.ed25519.key".to_string(),
            bytes: None,
        },
        cause: input.cause.clone(),
        correlation: Some([0; 32]),
    }];
    *ctx.directives = Some(serde_cbor::to_vec(&out_membrane).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_HPC: HpcEntry = HpcEntry { entry: hpc_entry };
