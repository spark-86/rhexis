use crate::View;

pub fn view(args: View) -> anyhow::Result<()> {
    let reader = std::fs::File::open(args.path)?;
    let reader = std::io::BufReader::new(reader);
    let flux_state: Vec<rhexis_core::flux::item::FluxItem> = serde_cbor::from_reader(reader)?;
    println!("{:?}", flux_state);
    Ok(())
}
