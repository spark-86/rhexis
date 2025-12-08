use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::flux::{meta::FluxMeta, payload::FluxPayload};
use base64::Engine;

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonFluxItem {
    pub name: String,
    pub schema: Option<String>,
    pub payload: JsonPayload,
    pub meta: FluxMeta,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum JsonPayload {
    Json {
        value: serde_json::Value,
    },

    Binary {
        encoding: JsonEncoding,
        data: Option<String>,
        path: Option<PathBuf>,
    },

    Mixed {
        json: serde_json::Value,
        blobs: Vec<JsonBlob>,
    },
}

impl JsonPayload {
    pub fn into_real(self) -> anyhow::Result<FluxPayload> {
        use JsonPayload::*;

        match self {
            Json { value } => Ok(FluxPayload::Json(value)),

            Binary {
                encoding,
                data,
                path,
            } => {
                let bytes = decode_blob(encoding, data, path)?;
                Ok(FluxPayload::Binary(bytes))
            }

            Mixed { json, blobs } => {
                let resolved = blobs
                    .into_iter()
                    .map(|b| decode_blob(b.encoding, b.data, b.path))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(FluxPayload::Mixed {
                    meta: json,
                    data: resolved,
                })
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonBlob {
    encoding: JsonEncoding,
    data: Option<String>,
    path: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum JsonEncoding {
    Base64,
    Hex,
    File,
}

fn decode_blob(
    encoding: JsonEncoding,
    data: Option<String>,
    path: Option<PathBuf>,
) -> anyhow::Result<Vec<u8>> {
    Ok(match encoding {
        JsonEncoding::Base64 => {
            let s = data.ok_or_else(|| anyhow::anyhow!("binary/base64 missing `data`"))?;
            let engine = base64::engine::general_purpose::STANDARD;
            engine.decode(&s)?
        }
        JsonEncoding::Hex => {
            let s = data.ok_or_else(|| anyhow::anyhow!("binary/hex missing `data`"))?;
            hex::decode(&s)?
        }
        JsonEncoding::File => {
            let p = path.ok_or_else(|| anyhow::anyhow!("binary/file missing `path`"))?;
            std::fs::read(&p)?
        }
    })
}
