use crate::transform::pattern::TransformPattern;

pub struct TransformSignature {
    pub id: String,
    pub observes: Vec<TransformPattern>,
    pub consumes: Vec<TransformPattern>,
    pub emits: Vec<TransformPattern>,
    pub proposes: Vec<TransformPattern>,
    pub requires: Vec<String>,
    pub accesses: Vec<String>,
}

impl TransformSignature {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            observes: vec![],
            consumes: vec![],
            emits: vec![],
            proposes: vec![],
            requires: vec![],
            accesses: vec![],
        }
    }
}
