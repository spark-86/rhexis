use rhexis_core::hpc::{context::HpcContext, entry::HpcEntry};

use crate::write::console_write;

mod write;

#[unsafe(no_mangle)]
extern "C" fn hpc_entry(ctx: &mut HpcContext) -> i32 {
    let s = std::str::from_utf8(&ctx.input).unwrap();
    let _ = console_write(s);
    0
}

#[unsafe(no_mangle)]
pub static RHEX_HPC: HpcEntry = HpcEntry { entry: hpc_entry };
