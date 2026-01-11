use serde::{Deserialize, Serialize};

use crate::rhex::{
    context::RhexContext,
    intent::RhexIntent,
    signature::{RhexSignature, SignatureType},
};

pub mod context;
pub mod intent;
pub mod payload;
pub mod signature;
pub mod states;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rhex {
    pub magic: [u8; 6],
    pub intent: RhexIntent,
    pub context: RhexContext,
    pub signatures: Vec<RhexSignature>,
    pub current_hash: Option<[u8; 32]>,
}

impl Rhex {
    pub fn new() -> Self {
        Self {
            magic: *b"RHEX01",
            intent: RhexIntent::new(RhexIntent::gen_nonce()),
            context: RhexContext::new(),
            signatures: Vec::new(),
            current_hash: None,
        }
    }

    pub fn calc_author_hash(&self) -> Result<[u8; 32], anyhow::Error> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.magic);
        hasher.update(&self.intent.as_bytes());
        let hash = hasher.finalize();
        Ok(*hash.as_bytes())
    }

    pub fn calc_usher_hash(&self) -> Result<[u8; 32], anyhow::Error> {
        let author_sig = self
            .signatures
            .iter()
            .find(|s| s.sig_type == SignatureType::Author)
            .ok_or(anyhow::anyhow!("No author signature found"))?;

        let mut hasher = blake3::Hasher::new();
        hasher.update(&author_sig.signature);
        hasher.update(&self.context.as_bytes());
        let hash = hasher.finalize();
        Ok(*hash.as_bytes())
    }

    pub fn calc_quorum_hash(&self) -> Result<[u8; 32], anyhow::Error> {
        let author_sig = self
            .signatures
            .iter()
            .find(|s| s.sig_type == SignatureType::Author)
            .ok_or(anyhow::anyhow!("No author signature found"))?;
        let usher_sig = self
            .signatures
            .iter()
            .find(|s| s.sig_type == SignatureType::Usher)
            .ok_or(anyhow::anyhow!("No usher signature found"))?;

        let mut hasher = blake3::Hasher::new();
        hasher.update(&author_sig.signature);
        hasher.update(&usher_sig.signature);
        let hash = hasher.finalize();
        Ok(*hash.as_bytes())
    }

    pub fn calc_final_hash(&mut self) -> Result<(), anyhow::Error> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.magic);
        hasher.update(&self.intent.as_bytes());
        hasher.update(&self.context.as_bytes());
        hasher.update(&serde_cbor::to_vec(&self.signatures)?);
        let hash = hasher.finalize();
        self.current_hash = Some(*hash.as_bytes());
        Ok(())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let bytes = serde_cbor::to_vec(&self);
        bytes.unwrap()
    }
}

impl Default for Rhex {
    fn default() -> Self {
        Self::new()
    }
}
