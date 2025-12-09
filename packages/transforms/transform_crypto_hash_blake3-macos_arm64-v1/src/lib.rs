use rhexis_core::transform::{context::TransformContext, entry::TransformEntry};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let data = ctx.input[0].payload.as_bytes();
    ctx.hpc_calls.push(("crypto.hash.blake3".to_string(), data));
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
