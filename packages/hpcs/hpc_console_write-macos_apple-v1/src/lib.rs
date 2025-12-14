use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta, payload::FluxPayload},
    hpc::{context::HpcContext, entry::HpcEntry},
};
use serde::{Deserialize, Serialize};

use crate::write::console_write;

mod write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleWrite {
    pub bytes: Vec<u8>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpc_entry(ctx: *mut HpcContext) -> i32 {
    let ctx = unsafe { &mut *ctx };

    // 1. Decode the CBOR into the correct shape
    let input: ConsoleWrite = match serde_cbor::from_slice(ctx.input) {
        Ok(v) => v,
        Err(e) => {
            // TODO: record diag / emit error flux instead of just returning
            eprintln!("hpc.console.write decode error: {e}");
            return 1; // non-zero = failure
        }
    };

    // 2. Turn the bytes into a string for macOS stdout
    let s = match String::from_utf8(input.bytes) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("hpc.console.write utf8 error: {e}");
            return 1;
        }
    };

    let _ = console_write(&s);
    let out_flux = FluxItem {
        name: "_console.write".to_string(),
        availability: FluxAvailability::Now,
        schema: None,
        payload: FluxPayload::Binary(ctx.input.to_vec()),
        meta: FluxMeta {
            creator: "hpc.console.write-macos_arm64-v1".to_string(),
            timestamp: 0,
        },
    };

    // 3. Emit flux (you can keep echoing ctx.input, that’s fine)
    *ctx.output = Some(serde_cbor::to_vec(&out_flux).unwrap());

    0
}

#[unsafe(no_mangle)]
pub static RHEX_HPC: HpcEntry = HpcEntry { entry: hpc_entry };
