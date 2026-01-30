use serde::{Deserialize, Serialize};

use crate::usher::Usher;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scope {
    pub name: String,
    pub policy: ScopePolicy,
    pub ushers: Vec<Usher>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScopePolicy {
    pub description: String,
    pub rules: Vec<Rule>,
    pub eff: u64,
    pub exp: u64,
    pub tags: Vec<String>,
    pub last_updated: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    pub append_roles: Vec<String>,
    pub k: u16,
    pub quorum_roles: Vec<String>,
    pub min_delay: u64,
    pub record_types: Vec<String>,
    pub window: u64,
}
