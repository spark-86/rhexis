pub struct TransformContext<'a> {
    pub input: &'a [u8],
    pub output: &'a mut Option<Vec<u8>>,
    pub diag: &'a mut Option<Vec<u8>>,
    pub hpc_calls: &'a mut Option<Vec<u8>>,
}
