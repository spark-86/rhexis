use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    hpc::{context::HpcContext, entry::HpcEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn hpc_entry(ctx: *mut HpcContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let mut hasher = blake3::Hasher::new();
    hasher.update(ctx.input);
    let out_flux = FluxItem {
        name: "hash.result".to_string(),
        availability: FluxAvailability::Now,
        schema: Some("rhex://schema/blake3-hash@1".to_string()),
        payload: rhexis_core::flux::payload::FluxPayload::Binary(
            hasher.finalize().as_bytes().to_vec(),
        ),
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
