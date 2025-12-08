use crate::transform::context::TransformContext;

#[repr(C)]
pub struct TransformEntry {
    pub entry: extern "C" fn(ctx: &mut TransformContext) -> i32,
}
