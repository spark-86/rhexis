use crate::hpc::context::HpcContext;

#[repr(C)]
pub struct HpcEntry {
    pub entry: extern "C" fn(ctx: &mut HpcContext) -> i32,
}
