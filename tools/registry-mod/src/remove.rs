use struct_registry::Registry;

use crate::Remove;

pub fn remove(args: Remove) {
    let input = &args.input;
    let entry = &args.entry;
    let output = &args.output;

    let file = std::fs::File::open(input).unwrap();
    let mut registry: Registry = serde_cbor::from_reader(file).unwrap();
    registry.remove(entry);
    let output_file = std::fs::File::create(output).unwrap();
    serde_cbor::to_writer(output_file, &registry).unwrap();
}
