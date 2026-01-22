use std::{io::Read, net::TcpListener, sync::Arc, thread};

use crossbeam::queue::SegQueue;
use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
};
use serde_cbor::Error;
use serde_json::json;

pub fn spawn_net_listener(addr: &str, queue: Arc<SegQueue<FluxItem>>) {
    let listener = TcpListener::bind(addr).expect("bind failed");

    println!("NET: listening on {}", addr);

    thread::spawn(move || {
        for stream in listener.incoming() {
            println!("NET: connection accepted");
            if let Ok(mut stream) = stream {
                let mut buf = Vec::new();
                if stream.read_to_end(&mut buf).is_ok() {
                    println!("NET: received {} bytes", buf.len());
                    if let Ok(flux) = decode_verify_flux(&buf) {
                        for f in flux {
                            println!("NET: decoded flux {}", f.name);
                            queue.push(f);
                        }
                    }
                }
            }
        }
    });
}

fn decode_verify_flux(buf: &Vec<u8>) -> anyhow::Result<Vec<FluxItem>> {
    let mut out_flux: Vec<FluxItem> = Vec::new();
    let decoded: Result<Vec<FluxItem>, Error> = serde_cbor::from_slice(&buf);

    if decoded.is_err() {
        let mut error_flux_intent = RhexIntent::new(RhexIntent::gen_nonce());
        error_flux_intent.schema = Binding::Bound("rhex://schema.system.net.error".to_string());
        error_flux_intent.data = RhexPayload::Json(json!({
            "error": "error.net.process.incoming",
            "message": "Could not decode incoming net traffic into flux."
        }));
        let hash = blake3::hash(buf);
        out_flux.push(FluxItem {
            // Bro it's too early in the morning to come up with a deterministic ID, a hash of the whole data.
            name: format!(
                "error.net.process.incoming.{}",
                hex::encode(hash.as_bytes())
            ),
            thread: "system.errors".to_string(),
            availability: FluxAvailability::Now,
            intent: error_flux_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "system.membrane".to_string(),
                timestamp: 0,
            },
        })
    } else {
        out_flux = decoded.unwrap();
    };

    return Ok(out_flux);
}
