use crate::Pack;
use rhexis_core::{
    flux::payload::PayloadType,
    rhp::descriptor::{
        BinaryFormat, BindTarget, HpcDescriptor, PatternDescriptor, RhpDescriptor,
        TransformDescriptor,
    },
};
use std::io::Write;

pub fn pack(args: Pack) {
    if args.plugin_type != "transform" && args.plugin_type != "hpc" {
        panic!("Invalid plugin type");
    }

    let code = std::fs::read(&args.code_path).unwrap();
    let descriptor_bytes = std::fs::read(&args.descriptor_path).unwrap();
    let desc_json = serde_json::from_slice::<serde_json::Value>(&descriptor_bytes).unwrap();

    let package = if args.plugin_type == "hpc" {
        build_hpc(&code, &desc_json)
    } else {
        build_transform(&code, &desc_json)
    };

    let outfile = std::fs::File::create(&args.output_path).unwrap();
    let mut writer = std::io::BufWriter::new(outfile);
    writer
        .write_all(&serde_cbor::to_vec(&package).unwrap())
        .unwrap();
}

//
// ──────────────────────────────────────────────────────────────────────────
//   HPC BUILDER
// ──────────────────────────────────────────────────────────────────────────
//

fn build_hpc(code: &[u8], json: &serde_json::Value) -> rhexis_core::rhp::package::RhpPackage {
    let mut desc = HpcDescriptor {
        descriptor_ver: json["descriptor"].as_u64().unwrap() as u32,
        name: json["name"].as_str().unwrap().to_owned(),
        capability: json["capability"].as_str().unwrap().to_owned(),
        version: json["version"].as_str().unwrap().to_owned(),
        requires: json["requires"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|x| x.as_str().unwrap().to_owned())
            .collect(),
        bin_format: match json["bin_format"].as_str() {
            Some("wasm") => BinaryFormat::Wasm,
            _ => BinaryFormat::Native,
        },
        blake3: [0; 32],
    };

    desc.blake3 = blake3::hash(code).into();

    rhexis_core::rhp::package::RhpPackage {
        kind: rhexis_core::rhp::kind::RhpKind::Hpc,
        descriptor: RhpDescriptor::Hpc(desc),
        binary: code.to_vec(),
    }
}

//
// ──────────────────────────────────────────────────────────────────────────
//   TRANSFORM BUILDER
// ──────────────────────────────────────────────────────────────────────────
//

fn build_transform(code: &[u8], json: &serde_json::Value) -> rhexis_core::rhp::package::RhpPackage {
    let mut desc = TransformDescriptor {
        descriptor_ver: json["descriptor"].as_u64().unwrap() as u32,
        name: json["name"].as_str().unwrap().to_owned(),
        version: json["version"].as_str().unwrap().to_owned(),
        requires: json["requires"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|x| x.as_str().unwrap().to_owned())
            .collect(),
        views: json["views"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|x| x.as_str().unwrap().to_owned())
            .collect(),
        interacts: load_patterns(json.get("interacts")),
        bind: load_binding(json.get("bind")),
        effects: load_patterns(json.get("effects")),
        bin_format: match json["bin_format"].as_str() {
            Some("wasm") => BinaryFormat::Wasm,
            _ => BinaryFormat::Native,
        },
        blake3: [0; 32],
    };

    desc.blake3 = blake3::hash(code).into();

    rhexis_core::rhp::package::RhpPackage {
        kind: rhexis_core::rhp::kind::RhpKind::Transform,
        descriptor: RhpDescriptor::Transform(desc),
        binary: code.to_vec(),
    }
}

//
// ──────────────────────────────────────────────────────────────────────────
//   PATTERN PARSER
// ──────────────────────────────────────────────────────────────────────────
//

fn load_patterns(node: Option<&serde_json::Value>) -> Vec<PatternDescriptor> {
    let Some(arr) = node.and_then(|v| v.as_array()) else {
        return vec![];
    };

    arr.iter()
        .map(|item| {
            let key = item
                .get("key")
                .and_then(|x| x.as_str())
                .map(|s| s.to_owned());
            let schema = item
                .get("schema")
                .and_then(|x| x.as_str())
                .map(|s| s.to_owned());
            println!("{:?}", item);
            let thread = item["thread"].as_str().unwrap().to_owned();
            let payload_type = item["payload_type"].as_str().unwrap().to_owned();
            let payload_type = match payload_type.as_str() {
                "json" => PayloadType::Json,
                "binary" => PayloadType::Binary,
                "mixed" => PayloadType::Mixed,
                "none" => PayloadType::None,
                _ => PayloadType::Any,
            };
            let required_fields = item
                .get("required_fields")
                .and_then(|x| x.as_array())
                .map(|arr| arr.iter().map(|v| v.as_str().unwrap().to_owned()).collect());
            let flags = item
                .get("flags")
                .and_then(|x| x.as_array())
                .map(|x| x.iter().map(|v| v.as_str().unwrap().to_owned()).collect())
                .unwrap_or(vec![]);
            PatternDescriptor {
                key,
                thread,
                schema,
                payload_type,
                required_fields,
                flags,
            }
        })
        .collect()
}

fn load_binding(node: Option<&serde_json::Value>) -> Option<BindTarget> {
    let obj = node?.as_object()?;

    let thread = obj.get("thread")?.as_str()?;
    let schema = obj.get("schema")?.as_str()?;

    Some(BindTarget {
        thread: thread.to_string(),
        schema: schema.to_string(),
    })
}
