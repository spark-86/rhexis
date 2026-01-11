use serde::{Deserialize, Serialize};

use crate::usher::Usher;

pub mod policy;
pub mod scope;
pub mod usher;

#[derive(Serialize, Deserialize, Clone)]
pub struct LatticeScopeRequest {
    pub scope: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LatticeScopeResponse {
    pub scope: String,
    pub location: LatticeScopeLocation,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LatticeLocation {
    pub scope: String,
    pub ushers: Vec<Usher>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum LatticeScopeLocation {
    Local(LatticeLocation),
    Remote(LatticeLocation),
    Unknown,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum LatticeScopeCacheAction {
    Add(LatticeScopeCacheAdd),
    Update(LatticeScopeCacheUpdate),
    Remove(LatticeScopeCacheRemove),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LatticeScopeCacheAdd {
    pub location: LatticeScopeLocation,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LatticeScopeCacheUpdate {
    pub location: LatticeScopeLocation,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LatticeScopeCacheRemove {
    pub scope: String,
}
