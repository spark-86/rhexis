use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use rhexis_core::rhp::package::RhpPackage;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    plugin_script: String,
}

fn main() {
    let args = Args::parse();

    let script = serde_json::from_str::<serde_json::Value>(
        &std::fs::read_to_string(args.plugin_script).unwrap(),
    )
    .unwrap();

    let rhp_list = script.get("plugins").unwrap();
    let rhp_list = rhp_list.as_array().unwrap();

    for rhp in rhp_list {
        let filename = rhp.as_str().unwrap();
        let rhp_bin = std::fs::read(filename).unwrap();
        let rhp_package = serde_cbor::from_slice::<RhpPackage>(&rhp_bin).unwrap();
        println!("{:?}", rhp_package.descriptor);
        let loaded_hpc = rhexis_membrane::loader::load_hpc_entry(
            &rhp_package,
            &PathBuf::from_str("./hpc_cache/").unwrap(),
        )
        .unwrap();
    }

    println!("{:?}", script);
}
