use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Policy {
    pub scope: String,
    pub rules: Vec<PolicyRule>,
    pub quorum_ttl: Option<u64>,
    pub effective_mm: Option<u64>,
    pub expiration_mm: Option<u64>,
    pub note: Option<String>,
}

impl Policy {
    pub fn new(scope: String) -> Self {
        Self {
            scope,
            rules: Vec::new(),
            quorum_ttl: None,
            effective_mm: None,
            expiration_mm: None,
            note: None,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "scope": self.scope.clone(),
            "rules": self.rules.clone(),
            "quorum_ttl": self.quorum_ttl.clone(),
            "effective_mm": self.effective_mm.clone(),
            "expiration_mm": self.expiration_mm.clone(),
            "note": self.note.clone(),
        })
    }

    pub fn from_json(json: serde_json::Value) -> Self {
        Self {
            scope: json["scope"].as_str().unwrap().to_string(),
            rules: json["rules"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| PolicyRule {
                    record_type: r["record_type"].as_str().unwrap().to_string(),
                    append_roles: r["append_roles"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|r| r.as_str().unwrap().to_string())
                        .collect(),
                    quorum_k: r["quorum_k"].as_u64().unwrap() as u8,
                    quorum_roles: r["quorum_roles"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|r| r.as_str().unwrap().to_string())
                        .collect(),
                    rate_per_mark: r["rate_per_mark"].as_u64().unwrap(),
                })
                .collect(),
            quorum_ttl: json["quorum_ttl"].as_u64(),
            effective_mm: json["effective_mm"].as_u64(),
            expiration_mm: json["expiration_mm"].as_u64(),
            note: json["note"].as_str().map(|s| s.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PolicyRule {
    pub record_type: String,
    pub append_roles: Vec<String>,
    pub quorum_k: u8,
    pub quorum_roles: Vec<String>,
    pub rate_per_mark: u64,
}

impl PolicyRule {
    pub fn new(record_type: String) -> Self {
        Self {
            record_type,
            append_roles: Vec::new(),
            quorum_k: 0,
            quorum_roles: Vec::new(),
            rate_per_mark: 0,
        }
    }
}
