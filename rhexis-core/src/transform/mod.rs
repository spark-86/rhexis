use crate::{flux::item::FluxItem, membrane::Membrane, transform::signature::TransformSignature};

pub mod pattern;
pub mod signature;

pub type TransformResult = Result<Option<Vec<FluxItem>>, TransformError>;

#[derive(Debug)]
pub struct TransformError {
    pub kind: String,
    pub message: String,
}

pub trait Transform {
    fn signature(&self) -> TransformSignature;
    fn run(
        &self,
        observed: Vec<&FluxItem>,
        consumed: Vec<&FluxItem>,
        membrane: &Box<dyn Membrane>,
    ) -> TransformResult;
}
