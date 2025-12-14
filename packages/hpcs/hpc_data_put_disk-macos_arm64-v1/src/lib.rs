use std::io::Write;

use rhexis_core::hpc::{
    context::{Fault, HpcContext},
    directive::{MembraneDirective, ResourceBacking},
    entry::HpcEntry,
    envelope::HpcCallEnvelope,
};
use serde::{Deserialize, Serialize};

#[cfg(unix)]
fn encode_fd_token(fd: i32) -> Vec<u8> {
    fd.to_le_bytes().to_vec()
}

#[derive(Debug, Serialize, Deserialize)]
struct DataReference {
    logical_id: Vec<u8>,
    data: Vec<u8>,
}

#[unsafe(no_mangle)]
pub extern "C" fn hpc_entry(ctx: *mut HpcContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let envelope: HpcCallEnvelope = serde_cbor::from_slice(ctx.input).unwrap();
    let logical_id = envelope.logical_id.clone();
    let input: DataReference = serde_cbor::from_slice(&envelope.payload).unwrap();
    // 2. Perform OS-level embodiment (HPC job)
    let filename = format!("/tmp/data/{}.rdat", hex::encode(&input.logical_id.clone()));

    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&filename)
    {
        Ok(f) => f,
        Err(e) => {
            ctx.diag.push(Fault {
                kind: "io_error".into(),
                message: format!("Open failed: {e}"),
            });
            return -1;
        }
    };

    if let Err(e) = file.write_all(&envelope.payload) {
        ctx.diag.push(Fault {
            kind: "io_error".into(),
            message: format!("Write failed: {e}"),
        });
        return -1;
    }
    let _ = file.flush();

    #[cfg(unix)]
    let token = {
        use std::os::fd::IntoRawFd;

        let fd = file.into_raw_fd();
        encode_fd_token(fd)
    };
    let dir = vec![MembraneDirective::RegisterResource {
        logical_id: input.logical_id.clone().to_vec(),
        token: token.clone(),
        backing: ResourceBacking {
            kind: "disk.file".to_string(),
            bytes: filename.into(),
        },
        cause: envelope.cause.clone(),
    }];
    // 3. Inform membrane of the new resource embodiment
    *ctx.directives = Some(serde_cbor::to_vec(&dir).unwrap());

    0
}

#[unsafe(no_mangle)]
pub static RHEX_HPC: HpcEntry = HpcEntry { entry: hpc_entry };
