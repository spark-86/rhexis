use serde_json::Value;

use crate::{Pack, parse};

pub fn pack(pack: Pack) -> anyhow::Result<()> {
    let input_path = pack.input;
    let output_path = pack.output;
    let flux_json = std::fs::read_to_string(input_path)?;
    let flux_json = serde_json::from_str::<Value>(&flux_json)?;
    let flux_state = parse::parse_flux_json(flux_json);
    let flux_bin = serde_cbor::to_vec(&flux_state)?;
    std::fs::write(output_path, flux_bin)?;
    Ok(())
}
