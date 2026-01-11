use crate::Generate;
use base64::{Engine, engine::general_purpose};
use rand::random;
use struct_registry::RegistryEntry;

pub fn generate(args: Generate) {
    let entry = &args.entry;
    let data_type = &args.data_type;
    let value = &args.value;
    if value.is_none() && data_type != "logical_id" {
        return;
    }
    let entry_data: RegistryEntry = match data_type.as_str() {
        "logical_id" => {
            if value.is_none() {
                let value: [u8; 32] = random();
                RegistryEntry::LogicalId(value)
            } else {
                let value: [u8; 32] = general_purpose::STANDARD
                    .decode(value.clone().unwrap())
                    .unwrap()
                    .try_into()
                    .unwrap();
                RegistryEntry::LogicalId(value)
            }
        }
        "u64" => {
            let value: u64 = value.clone().unwrap().parse().unwrap();
            RegistryEntry::U64(value)
        }
        "string" => RegistryEntry::String(value.clone().unwrap().clone()),
        _ => return,
    };

    let out_string = general_purpose::STANDARD.encode(serde_cbor::to_vec(&entry_data).unwrap());
    println!("{}: {}", entry, out_string);
}
