use base64::{Engine, engine::general_purpose};
use std::fs::read_to_string;
use struct_lattice::{
    scope::Scope,
    usher::{Usher, UsherLocation},
};

use crate::Pack;

pub fn pack_cache(args: Pack) -> Result<(), anyhow::Error> {
    let input = args.input;
    let output = args.output;
    let mut out_scopes: Vec<Scope> = Vec::new();

    // load file
    let src = read_to_string(input)?;
    let src = serde_json::from_str::<serde_json::Value>(&src)?;

    for scope in src.as_array().unwrap() {
        // Convert ushers from base64 to bytes
        let mut ushers: Vec<Usher> = Vec::new();
        for u in scope["ushers"].as_array().unwrap() {
            let loc = match u["location"]["distance"].as_str().unwrap() {
                "local" => UsherLocation::Local,
                "remote" => UsherLocation::Remote {
                    ip_addr: u["location"]["ip_addr"].as_str().unwrap().to_string(),
                    port: u["location"]["port"].as_u64().unwrap() as u16,
                },
                _ => {
                    eprintln!(
                        "Unknown location distance: {}",
                        u["location"]["distance"].as_str().unwrap()
                    );
                    return Err(anyhow::anyhow!("Unknown location distance"));
                }
            };
            ushers.push(Usher {
                name: u["name"].as_str().unwrap().to_string(),
                public_key: general_purpose::URL_SAFE_NO_PAD
                    .decode(u["public_key"].as_str().unwrap())
                    .unwrap()
                    .try_into()
                    .unwrap(),
                priority: u["priority"].as_u64().unwrap() as u8,
                location: loc,
                last_updated: 0,
            });
        }
        out_scopes.push(Scope {
            name: scope["name"].as_str().unwrap().to_string(),
            policy: serde_json::from_value(scope["policy"].clone())?,
            ushers,
        });
    }

    // serialize to CBOR
    let bin = serde_cbor::to_vec(&out_scopes)?;
    std::fs::write(output, bin)?;

    Ok(())
}
