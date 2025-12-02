use crate::View;

pub fn view(args: View) {
    let reader = std::fs::File::open(args.path.clone()).unwrap();
    let reader = std::io::BufReader::new(reader);
    let package: rhexis_core::rhp::package::RhpPackage = serde_cbor::from_reader(reader).unwrap();
    println!("{:?}", package.descriptor);
}
