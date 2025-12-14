pub struct RhexSignature {
    pub sig_type: SignatureType,
    pub public_key: [u8; 32],
    pub signature: [u8; 64],
}

pub enum SignatureType {
    Author = 1,
    Usher = 2,
    Quorum = 3,
}
