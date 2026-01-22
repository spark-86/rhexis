use std::{collections::HashMap, io::Write, net::TcpStream};

use rhexis_core::{
    flux::item::FluxItem,
    hpc::{context::HpcContext, entry::HpcEntry, envelope::HpcCallEnvelope},
};
use struct_net::NetFlux;

#[unsafe(no_mangle)]
pub extern "C" fn hpc_entry(ctx: *mut HpcContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: HpcCallEnvelope = serde_cbor::from_slice(ctx.input).unwrap();

    let outbound_flux: Vec<NetFlux> = serde_cbor::from_slice(&input.payload).unwrap();

    let mut outbound_map: HashMap<String, Vec<FluxItem>> = HashMap::new();

    for flux in outbound_flux {
        if outbound_map.contains_key(format!("{}:{}", &flux.ip_addr, &flux.port).as_str()) {
            outbound_map
                .get_mut(format!("{}:{}", &flux.ip_addr, &flux.port).as_str())
                .unwrap()
                .extend(flux.payload);
        } else {
            outbound_map.insert(
                format!("{}:{}", &flux.ip_addr, &flux.port).to_string(),
                flux.payload,
            );
        }
    }

    for (key, value) in outbound_map {
        let mut socket = TcpStream::connect(&key).unwrap();
        socket
            .write_all(&serde_cbor::to_vec(&value).unwrap())
            .unwrap();
        socket.flush().unwrap();
        socket.shutdown(std::net::Shutdown::Both).unwrap();
        println!("Sent {} flux items to {}", value.len(), &key);
    }

    0
}

#[unsafe(no_mangle)]
pub static RHEX_HPC: HpcEntry = HpcEntry { entry: hpc_entry };
