use crate::Unpack;
use std::io::Write;

pub fn unpack(args: Unpack) {
    let reader = std::fs::File::open(args.input.clone()).unwrap();
    let reader = std::io::BufReader::new(reader);
    let package: rhexis_core::rhp::package::RhpPackage = serde_cbor::from_reader(reader).unwrap();
    let descriptor = serde_json::to_string(&package.descriptor).unwrap();
    let code = package.binary;
    let mut writer = std::fs::File::create(args.output.clone() + ".bin").unwrap();
    writer.write_all(&code).unwrap();
    let mut writer = std::fs::File::create(args.output.clone() + ".json").unwrap();
    writer.write_all(descriptor.as_bytes()).unwrap();
    println!("{}", descriptor);
}
