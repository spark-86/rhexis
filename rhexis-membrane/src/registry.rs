use std::collections::HashMap;

use libloading::Library;
use rhexis_core::{
    hpc::entry::HpcEntry,
    rhp::descriptor::{HpcDescriptor, TransformDescriptor},
};

pub struct LoadedHpc {
    pub descriptor: HpcDescriptor,
    pub entry: &'static HpcEntry,
    pub library: Library,
}

pub struct LoadedTransform {
    pub descriptor: TransformDescriptor,
}

pub struct MembraneRegistry {
    pub transforms: HashMap<String, LoadedTransform>,
    pub hpcs: HashMap<String, LoadedHpc>,
}
