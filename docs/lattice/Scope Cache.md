# Scope Cache

## What A Scope Record Looks Like

```rust
pub struct Scope {
    pub name: String,
    pub policy: ScopePolicy,
    pub ushers: Vec<Usher>,
}

pub struct ScopePolicy {
    pub description: String,
    pub rules: Vec<Rule>,
    pub eff: u64,
    pub exp: u64,
    pub tags: Vec<String>,
    pub last_updated: u64,
}

pub struct Rule {
    pub append_roles: Vec<String>,
    pub k: u16,
    pub quorum_roles: Vec<String>,
    pub min_delay: u64,
    pub record_types: Vec<String>,
    pub window: u64,
}


```

## What A Scope Does

Scopes are the natural division of the lattice. It allows the lattice to expand infinitely across namespaces. Scopes also enforce policy, which dictates who can and can't append to a scope.

## Building a New Scope
