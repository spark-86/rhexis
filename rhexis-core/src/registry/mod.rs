use std::{collections::HashMap, sync::Arc};

use crate::{
    hpc::entry::HpcEntry,
    rhp::descriptor::{HpcDescriptor, TransformDescriptor},
    transform::entry::TransformEntry,
};
use libloading::Library;

pub struct LoadedHpc {
    pub descriptor: HpcDescriptor,
    pub entry: &'static HpcEntry,
    pub library: Library,
}

pub struct LoadedTransform {
    pub descriptor: TransformDescriptor,
    pub entry: &'static TransformEntry,
    pub library: Library,
}

pub struct MembraneRegistry {
    pub transforms: Vec<Arc<LoadedTransform>>,
    pub hpcs: HashMap<String, LoadedHpc>,
}
