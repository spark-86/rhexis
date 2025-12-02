pub struct CapabilityDescriptor {
    pub id: &'static str,
    pub version: &'static str,
    pub requires: Vec<&'static str>,
    pub provides: &'static str,
    pub blake3: [u8; 32],
}

impl CapabilityDescriptor {
    pub fn new(
        id: &'static str,
        version: &'static str,
        requires: Vec<&'static str>,
        provides: &'static str,
    ) -> Self {
        Self {
            id,
            version,
            requires,
            provides,
            blake3: [0; 32],
        }
    }
}
