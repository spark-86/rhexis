use libloading::{Library, Symbol};
use std::fs;
use std::path::PathBuf;

use rhexis_core::hpc::entry::HpcEntry;
use rhexis_core::rhp::descriptor::{HpcDescriptor, RhpDescriptor};
use rhexis_core::rhp::package::RhpPackage;

/// Loads a native HPC from an RHP package, writes the binary to disk cache,
/// dlopens it, and returns the (descriptor, entrypoint reference, library).
///
/// IMPORTANT: The returned HpcEntry is a reference into the loaded library.
/// The Library must be stored to keep the HPC alive.
pub fn load_hpc_entry(
    pkg: &RhpPackage,
    cache_dir: &PathBuf,
) -> anyhow::Result<(HpcDescriptor, &'static HpcEntry, Library)> {
    // --- 1. Validate package kind ---
    let desc = match &pkg.descriptor {
        RhpDescriptor::Hpc(h) => h.clone(),
        _ => anyhow::bail!("Attempted to load a non-HPC package as HPC"),
    };

    // --- 2. Compute cache path from blake3 hash ---
    let hash_hex = hex::encode(desc.blake3);
    let mut path = cache_dir.clone();
    path.push(format!("{}.dylib", hash_hex));

    // --- 3. Write binary to cache if missing ---
    if !path.exists() {
        fs::create_dir_all(cache_dir)?;
        fs::write(&path, &pkg.binary)?;
    }

    // --- 4. dlopen the library ---
    let lib = unsafe { Library::new(&path) }?;

    // --- 5. Load the single HPC entry symbol ---
    //
    // extern "C" { pub static RHEX_HPC: HpcEntry; }
    //
    let symbol: Symbol<*const HpcEntry> = unsafe { lib.get(b"RHEX_HPC")? };

    // Safety: symbol points into the loaded library which we are returning.
    let entry_ref: &'static HpcEntry = unsafe { &**symbol };

    // --- 6. Return descriptor + reference + library ---
    Ok((desc, entry_ref, lib))
}
