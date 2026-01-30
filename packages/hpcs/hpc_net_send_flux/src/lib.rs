use std::{io::Write, net::TcpStream};

use rhexis_core::hpc::{context::HpcContext, entry::HpcEntry, envelope::HpcCallEnvelope};
use struct_net::NetFlux;

#[unsafe(no_mangle)]
pub extern "C" fn hpc_entry(ctx: *mut HpcContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: HpcCallEnvelope = serde_cbor::from_slice(ctx.input).unwrap();

    let net_flux: NetFlux = serde_cbor::from_slice(&input.payload).unwrap();

    let mut socket =
        TcpStream::connect(format!("{}:{}", &net_flux.ip_addr, &net_flux.port)).unwrap();
    socket
        .write_all(&serde_cbor::to_vec(&net_flux).unwrap())
        .unwrap();
    socket.flush().unwrap();
    socket.shutdown(std::net::Shutdown::Both).unwrap();
    println!(
        "Sent {} flux items to {}:{}",
        net_flux.payload.len(),
        &net_flux.ip_addr,
        &net_flux.port
    );

    0
}

#[unsafe(no_mangle)]
pub static RHEX_HPC: HpcEntry = HpcEntry { entry: hpc_entry };
