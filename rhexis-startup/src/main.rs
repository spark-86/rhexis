use std::{path::PathBuf, str::FromStr, sync::Arc};

use clap::Parser;
use rhexis_core::flux::item::FluxItem;
use rhexis_core::registry::{LoadedHpc, LoadedTransform};
use rhexis_core::rhp::{kind::RhpKind, package::RhpPackage};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    plugin_script: String,
    #[arg(short, long)]
    flux_store: String,
}

fn main() {
    let args = Args::parse();
    let mut hpc_vec = Vec::new();
    let mut transform_vec: Vec<Arc<LoadedTransform>> = Vec::new();

    // Load plugin script JSON
    let script = serde_json::from_str::<serde_json::Value>(
        &std::fs::read_to_string(&args.plugin_script).unwrap(),
    )
    .unwrap();

    // Load frozen flux state
    let flux_path = PathBuf::from_str(&args.flux_store).unwrap();
    let flux_bin = std::fs::read(flux_path).unwrap();
    let flux = serde_cbor::from_slice::<Vec<FluxItem>>(&flux_bin).unwrap();

    // Extract plugin RHP list
    let rhp_list = script.get("plugins").unwrap().as_array().unwrap();

    for rhp in rhp_list {
        let filename = rhp.as_str().unwrap();
        let rhp_bin = std::fs::read(filename).unwrap();
        let rhp_package = serde_cbor::from_slice::<RhpPackage>(&rhp_bin).unwrap();

        println!("{:?}", rhp_package.descriptor);

        match rhp_package.kind {
            RhpKind::Hpc => {
                let loaded = rhexis_membrane::loader::load_hpc_entry(
                    &rhp_package,
                    &PathBuf::from_str("./hpc_cache/").unwrap(),
                )
                .unwrap();

                hpc_vec.push(LoadedHpc {
                    descriptor: loaded.0,
                    entry: loaded.1,
                    library: loaded.2,
                });
            }

            RhpKind::Transform => {
                let loaded = rhexis_membrane::loader::load_transform_entry(
                    &rhp_package,
                    &PathBuf::from_str("./transform_cache/").unwrap(),
                )
                .unwrap();

                // ✔ Wrap transforms in Arc
                transform_vec.push(Arc::new(LoadedTransform {
                    descriptor: loaded.0,
                    entry: loaded.1,
                    library: loaded.2,
                }));
            }
        }
    }

    // Construct membrane from the loaded transforms and HPCs
    let membrane = rhexis_membrane::Membrane::new(transform_vec, hpc_vec);

    // Spin the kernel using the hydrated flux
    let _ = membrane.spin_kernel(&flux);
}
