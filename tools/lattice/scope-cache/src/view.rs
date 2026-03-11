use std::collections::HashMap;

use struct_lattice::scope::Scope;

use crate::View;

pub fn view_cache(args: View) -> anyhow::Result<()> {
    let reader = std::fs::File::open(args.path)?;
    let reader = std::io::BufReader::new(reader);
    let table: HashMap<String, Scope> = serde_cbor::from_reader(reader)?;
    println!("{:?}", table);
    Ok(())
}
