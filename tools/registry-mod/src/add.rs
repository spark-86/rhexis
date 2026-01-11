use std::collections::HashMap;

use crate::Add;
use base64::{Engine as _, engine::general_purpose};
use struct_registry::{Registry, RegistryEntry};

pub fn add(args: Add) {
    let new = &args.new;
    let input = &args.input;
    let entry = &args.entry;
    let data_type = &args.data_type;
    let value = &args.value;
    let output = &args.output;

    let mut registry: Registry = HashMap::new();

    if !new {
        if input.is_none() {
            return;
        }
        let input_file = std::fs::File::open(input.clone().unwrap()).unwrap();
        registry = serde_cbor::from_reader(input_file).unwrap();
    }
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
    println!("Final Registry: {:?}", registry);
    let output_file = std::fs::File::create(output).unwrap();
    serde_cbor::to_writer(output_file, &registry).unwrap();
}
