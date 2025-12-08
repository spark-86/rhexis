use crate::{flux::item::FluxItem, transform::context::TransformContext};

pub mod context;
pub mod entry;
pub mod io;
pub mod pattern;
pub mod signature;
pub mod transform_return;

pub type TransformResult = Result<Option<Vec<FluxItem>>, TransformError>;

#[derive(Debug)]
pub struct TransformError {
    pub kind: String,
    pub message: String,
}

pub trait Transform {
    fn run(&self, ctx: &mut TransformContext) -> TransformResult;
}
