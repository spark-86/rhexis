use crate::flux::item::FluxItem;

pub struct HpcContext<'a> {
    pub input: &'a [u8],
    pub flux_out: &'a mut Vec<FluxItem>,
    pub diag: &'a mut DiagSink,
}

pub struct DiagSink {
    pub debug: Vec<String>,
}
