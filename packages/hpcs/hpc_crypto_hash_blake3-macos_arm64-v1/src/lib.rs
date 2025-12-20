use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    hpc::{context::HpcContext, entry::HpcEntry, envelope::HpcCallEnvelope},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
};

#[unsafe(no_mangle)]
pub extern "C" fn hpc_entry(ctx: *mut HpcContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: HpcCallEnvelope = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut hasher = blake3::Hasher::new();
    hasher.update(&input.payload);
    let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
    intent.schema = Binding::Bound("rhex://schema/blake3-hash@1".to_string());
    intent.data = RhexPayload::Binary {
        data: hasher.finalize().as_bytes().to_vec(),
    };
    let out_flux = FluxItem {
        name: "hash.result".to_string(),
        thread: input.thread.clone(),
        availability: FluxAvailability::Now,
        intent,
        correlation: None,
        meta: FluxMeta {
            creator: "hpc.crypto.hash.blake3".to_string(),
            timestamp: 0,
        },
    };

    *ctx.output = Some(serde_cbor::to_vec(&out_flux).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_HPC: HpcEntry = HpcEntry { entry: hpc_entry };
