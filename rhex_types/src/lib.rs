pub mod context;
pub mod intent;
pub mod signature;

pub struct Rhex {
    pub magic: [u8; 6],
    pub intent: intent::RhexIntent,
    pub context: context::RhexContext,
    pub signatures: Vec<signature::RhexSignature>,
    pub current_hash: Option<[u8; 32]>,
}
