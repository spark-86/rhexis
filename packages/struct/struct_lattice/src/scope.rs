use serde::{Deserialize, Serialize};

use crate::usher::Usher;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scope {
    pub scope: String,
    pub policy: ScopePolicy,
    pub ushers: Vec<Usher>,
}

impl Scope {
    pub fn new(scope: &str, policy: ScopePolicy, ushers: Vec<Usher>) -> Self {
        Self {
            scope: scope.to_string(),
            policy,
            ushers,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScopePolicy {
    pub description: String,
    pub roles: ScopeRoles,
    pub quorum: ScopeQuorum,
    pub rate: ScopeRate,
    pub allowed: Vec<String>,
    pub tags: Vec<String>,
    pub last_updated: u64,
}

impl ScopePolicy {
    pub fn new(
        description: &str,
        roles: ScopeRoles,
        quorum: ScopeQuorum,
        rate: ScopeRate,
        allowed: Vec<String>,
        tags: Vec<String>,
    ) -> Self {
        Self {
            description: description.to_string(),
            roles,
            quorum,
            rate,
            allowed,
            tags,
            last_updated: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScopeRoles {
    pub read: Vec<String>,
    pub append: Vec<String>,
    pub keymaster: Vec<String>,
    pub quorum: Vec<String>,
}

impl ScopeRoles {
    pub fn new() -> Self {
        Self {
            read: Vec::new(),
            append: Vec::new(),
            keymaster: Vec::new(),
            quorum: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScopeQuorum {
    pub k: u16,
    pub window: u64,
}

impl ScopeQuorum {
    pub fn new(k: u16, window: u64) -> Self {
        Self { k, window }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScopeRate {
    pub waiting_period: u64,
    pub throughput: u64,
}

impl ScopeRate {
    pub fn new(waiting_period: u64, throughput: u64) -> Self {
        Self {
            waiting_period,
            throughput,
        }
    }
}
