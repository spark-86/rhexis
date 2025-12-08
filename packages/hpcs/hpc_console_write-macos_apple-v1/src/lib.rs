use rhexis_core::{
    flux::{meta::FluxMeta, payload::FluxPayload},
    hpc::{context::HpcContext, entry::HpcEntry},
};
use serde::{Deserialize, Serialize};

use crate::write::console_write;

mod write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleWrite {
    pub bytes: String,
}

#[unsafe(no_mangle)]
extern "C" fn hpc_entry(ctx: &mut HpcContext) -> i32 {
    let input: ConsoleWrite = serde_cbor::from_slice(ctx.input).unwrap();
    let s = std::str::from_utf8(&input.bytes.as_bytes()).unwrap();
    let _ = console_write(s);
    ctx.output.push(rhexis_core::flux::item::FluxItem {
        name: "_console.write".to_string(),
        schema: None,
        payload: FluxPayload::Binary(ctx.input.to_vec()),
        meta: FluxMeta {
            creator: "hpc.console.write-macos_apple-v1".to_string(),
            timestamp: 0,
        },
    });
    0
}

#[unsafe(no_mangle)]
pub static RHEX_HPC: HpcEntry = HpcEntry { entry: hpc_entry };
