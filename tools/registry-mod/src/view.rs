use struct_registry::Registry;

use crate::View;

pub fn view(args: View) {
    let reader = std::fs::File::open(args.input).unwrap();
    let reader = std::io::BufReader::new(reader);
    let flux_state: Registry = serde_cbor::from_reader(reader).unwrap();
    println!("{:?}", flux_state);
}
