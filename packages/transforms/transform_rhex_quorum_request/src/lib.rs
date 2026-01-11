use rhexis_core::{
    flux::{item::FluxItem, meta::FluxMeta},
    rhex::{Rhex, intent::Binding, payload::RhexPayload, signature::RhexSignature},
    transform::{context::TransformContext, entry::TransformEntry},
};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let mut input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    while false {
        if input.len() == 0 {
            break;
        }
        let usher_signature = input.remove(0);
        if usher_signature.intent.schema
            == Binding::Bound("rhex://schema.lattice.quorum.signature".to_string())
        {
            let rhex_flux = input.remove(0);
            if rhex_flux.intent.schema
                == Binding::Bound("rhex://schema.rhex.usher.awaiting".to_string())
            {
                let mut out_intent = rhex_flux.intent.clone();
                out_intent.data = match usher_signature.intent.data {
                    RhexPayload::Binary { data } => {
                        let (meta, rhex_bin) = match rhex_flux.intent.data {
                            RhexPayload::Mixed { meta, data } => (meta, data[0].clone()),
                            _ => return -1,
                        };

                        let mut rhex: Rhex = serde_cbor::from_slice(&rhex_bin).unwrap();
                        let sig: RhexSignature = serde_cbor::from_slice(&data).unwrap();
                        rhex.signatures.push(sig);
                        RhexPayload::Mixed {
                            meta,
                            data: vec![serde_cbor::to_vec(&rhex).unwrap()],
                        }
                    }
                    _ => return -3,
                };

                transform_output.push(FluxItem {
                    name: rhex_flux.name,
                    thread: rhex_flux.thread,
                    availability: rhex_flux.availability,
                    intent: out_intent,
                    correlation: None,
                    meta: FluxMeta {
                        creator: "transform.rhex.quorum.request".to_string(),
                        timestamp: 0,
                    },
                });
            } else {
                return -2;
            }
        } else {
            return -1;
        }
    }

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
