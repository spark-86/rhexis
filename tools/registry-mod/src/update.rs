use base64::{Engine, engine::general_purpose};
use struct_registry::{Registry, RegistryEntry};

use crate::Update;

pub fn update(args: Update) {
    let input = &args.input;
    let entry = &args.entry;
    let data_type = &args.data_type;
    let value = &args.value;
    let output = &args.output;

    let file = std::fs::File::open(input).unwrap();
    let mut registry: Registry = serde_cbor::from_reader(file).unwrap();

    let entry_data: RegistryEntry = match data_type.as_str() {
        "logical_id" => {
            let value: [u8; 32] = general_purpose::STANDARD
                .decode(value)
                .unwrap()
                .try_into()
                .unwrap();
            RegistryEntry::LogicalId(value)
        }
        "u64" => {
            let value: u64 = value.parse().unwrap();
            RegistryEntry::U64(value)
        }
        "string" => RegistryEntry::String(value.clone()),
        _ => return,
    };

    registry.insert(entry.clone(), entry_data);

    let output_file = std::fs::File::create(output).unwrap();
    serde_cbor::to_writer(output_file, &registry).unwrap();
}
