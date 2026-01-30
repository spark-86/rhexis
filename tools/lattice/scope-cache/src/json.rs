use serde::{Deserialize, Serialize};
use struct_lattice::scope::ScopePolicy;

#[derive(Serialize, Deserialize)]
pub struct JsonScopeRecord {
    scope: String,
    policy: ScopePolicy,
    ushers: Vec<JsonUsher>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonUsher {
    name: String,
    public_key: String,
    priority: u8,
    location: JsonUsherLocation,
    last_updated: u64,
}

#[derive(Serialize, Deserialize)]
pub struct JsonUsherLocation {
    distance: String,
    ip_addr: Option<String>,
    port: Option<u16>,
}
