use crate::Unpack;

pub fn unpack(args: Unpack) -> anyhow::Result<()> {
    let reader = std::fs::File::open(args.input)?;
    let reader = std::io::BufReader::new(reader);
    let flux_state: Vec<rhexis_core::flux::item::FluxItem> = serde_cbor::from_reader(reader)?;
    let flux_json = serde_json::to_string(&flux_state)?;
    std::fs::write(args.output, flux_json)?;
    Ok(())
}
