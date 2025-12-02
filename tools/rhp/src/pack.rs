use crate::Pack;
use rhexis_core::rhp::descriptor::{BinaryFormat, HpcDescriptor, TransformDescriptor};
use std::io::Write;

pub fn pack(args: Pack) {
    if args.plugin_type != "transform" && args.plugin_type != "hpc" {
        panic!("Invalid plugin type");
    }
    let code = std::fs::read(args.code_path).unwrap();
    let descriptor = std::fs::read(args.descriptor_path).unwrap();
    let desc_json = serde_json::from_slice::<serde_json::Value>(&descriptor).unwrap();
    let mut hpc_desc = HpcDescriptor {
        name: String::new(),
        capability: String::new(),
        version: String::new(),
        requires: Vec::new(),
        bin_format: BinaryFormat::Native,
        blake3: [0; 32],
    };
    let mut transform_desc = TransformDescriptor {
        name: String::new(),
        version: String::new(),
        requires: Vec::new(),
        observes: Vec::new(),
        consumes: Vec::new(),
        emits: Vec::new(),
        proposes: Vec::new(),
        bin_format: BinaryFormat::Native,
        blake3: [0; 32],
    };

    if args.plugin_type == "hpc" {
        hpc_desc.name = desc_json["name"].as_str().unwrap().to_string();
        hpc_desc.capability = desc_json["capability"].as_str().unwrap().to_string();
        hpc_desc.version = desc_json["version"].as_str().unwrap().to_string();
        hpc_desc.requires = desc_json["requires"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        if desc_json["bin_format"].as_str().unwrap() == "wasm" {
            hpc_desc.bin_format = BinaryFormat::Wasm;
        }
    } else {
        transform_desc.name = desc_json["name"].as_str().unwrap().to_string();
        transform_desc.version = desc_json["version"].as_str().unwrap().to_string();
        transform_desc.requires = desc_json["requires"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        transform_desc.observes = desc_json["observes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let key = x["key"].as_str();
                let mut key_str = None;
                let schema = x["schema"].as_str();
                let mut schema_str = None;
                let payload_type = x["payload_type"].as_str();
                let required_fields = x["required_fields"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect();
                if key.is_some() {
                    key_str = Some(key.unwrap().to_string());
                }
                if schema.is_some() {
                    schema_str = Some(schema.unwrap().to_string());
                }
                rhexis_core::rhp::descriptor::PatternDescriptor {
                    key: key_str,
                    schema: schema_str,
                    payload_type: payload_type.unwrap().to_string(),
                    required_fields: Some(required_fields),
                }
            })
            .collect();
    }

    let mut hasher = blake3::Hasher::new();
    hasher.update(&code);
    if args.plugin_type == "hpc" {
        hpc_desc.blake3 = hasher.finalize().into();
    } else {
        transform_desc.blake3 = hasher.finalize().into();
    }

    let outfile = std::fs::File::create(args.output_path).unwrap();
    let mut writer = std::io::BufWriter::new(outfile);

    if args.plugin_type == "hpc" {
        let hpc_package = rhexis_core::rhp::package::RhpPackage {
            kind: rhexis_core::rhp::kind::RhpKind::Hpc,
            descriptor: rhexis_core::rhp::descriptor::RhpDescriptor::Hpc(hpc_desc),
            binary: code,
        };
        let serialized = serde_cbor::to_vec(&hpc_package).unwrap();
        writer.write_all(&serialized).unwrap();
    } else {
        let transform_package = rhexis_core::rhp::package::RhpPackage {
            kind: rhexis_core::rhp::kind::RhpKind::Transform,
            descriptor: rhexis_core::rhp::descriptor::RhpDescriptor::Transform(transform_desc),
            binary: code,
        };
        let serialized = serde_cbor::to_vec(&transform_package).unwrap();
        writer.write_all(&serialized).unwrap();
    }
}
