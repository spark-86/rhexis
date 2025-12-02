use rhexis_core::hpc::{context::HpcContext, entry::HpcEntry};

pub const CAPABILITY: &str = "time.unix.now_ms";

#[unsafe(no_mangle)]
extern "C" fn hpc_entry(ctx: &mut HpcContext) -> i32 {
    let _ = ctx;
    0
}

#[unsafe(no_mangle)]
pub static RHEX_HPC: HpcEntry = HpcEntry { entry: hpc_entry };
