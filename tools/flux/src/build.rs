use std::path::PathBuf;

use anyhow::Context;
use base64::Engine;
use rhexis_core::{
    flux::{availability::FluxAvailability, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct FluxBuildSpec {
    pub name: String,
    pub thread: String,
    pub availability: FluxAvailability,
    pub intent: IntentBuildSpec,
    pub correlation: Option<[u8; 32]>,
    pub meta: FluxMeta,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct IntentBuildSpec {
    #[serde(default)]
    pub previous_hash: Binding<[u8; 32]>,
    #[serde(default)]
    pub scope: Binding<String>,
    #[serde(default = "RhexIntent::gen_nonce")]
    pub nonce: [u8; 32],
    #[serde(default)]
    pub author_public_key: Binding<[u8; 32]>,
    #[serde(default)]
    pub usher_public_key: Binding<[u8; 32]>,
    #[serde(default)]
    pub schema: Binding<String>,
    #[serde(default)]
    pub record_type: Binding<String>,
    pub data: BuildPayload,
}

#[derive(Deserialize)]
#[serde(tag = "kind", content = "value")]
pub enum BuildPayload {
    Json(serde_json::Value),
    Binary(JsonEncoding),
    Mixed {
        meta: serde_json::Value,
        data: Vec<BuildPayload>,
    },
    None,
}

impl BuildPayload {
    pub fn materialize(self) -> anyhow::Result<RhexPayload> {
        match self {
            BuildPayload::Json(v) => Ok(RhexPayload::Json(v)),
            BuildPayload::Binary(encoding) => Ok(RhexPayload::Binary {
                data: decode_blob(encoding)?,
            }),
            BuildPayload::Mixed { meta, data } => {
                let resolved: Vec<Vec<u8>> = data
                    .into_iter()
                    .map(|payload| match payload.materialize()? {
                        RhexPayload::Binary { data: v } => Ok(v),
                        other => Err(anyhow::anyhow!(
                            "mixed payload elements must materialize to Binary, got {:?}",
                            other
                        )),
                    })
                    .collect::<Result<_, _>>()?;
                Ok(RhexPayload::Mixed {
                    meta,
                    data: resolved,
                })
            }
            BuildPayload::None => Ok(RhexPayload::None),
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "encoding", content = "data", rename_all = "lowercase")]
pub enum JsonEncoding {
    Base64(String),
    File(PathBuf),
    Array(Vec<u8>),
}

fn decode_blob(encoding: JsonEncoding) -> anyhow::Result<Vec<u8>> {
    match encoding {
        JsonEncoding::Base64(data) => {
            let engine = base64::engine::general_purpose::STANDARD;
            engine
                .decode(data.as_bytes())
                .context("invalid base64 data")
        }

        JsonEncoding::File(path) => {
            std::fs::read(&path).with_context(|| format!("failed to read file {:?}", path))
        }

        JsonEncoding::Array(data) => Ok(data),
    }
}

pub fn build(args: crate::Build) -> anyhow::Result<()> {
    let input_path = args.input;
    let output_path = args.output;
    let rhex_payload = std::fs::read(input_path)?;
    let rhex_payload: Value = serde_json::from_slice::<Value>(&rhex_payload)?;
    let json_value = rhex_payload.clone();
    let rhex_payload = serde_json::from_value::<BuildPayload>(json_value.clone())?;
    let rhex_payload = rhex_payload.materialize()?;
    println!("{}", serde_json::to_string_pretty(&json_value)?);
    println!("{:?}", rhex_payload);
    let engine = base64::engine::general_purpose::STANDARD;
    println!("{}", engine.encode(rhex_payload.as_bytes()));
    let rhex_payload_bin = serde_cbor::to_vec(&rhex_payload)?;
    std::fs::write(output_path, rhex_payload_bin)?;
    Ok(())
}
