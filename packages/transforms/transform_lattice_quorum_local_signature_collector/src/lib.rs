use rhexis_core::{
    flux::item::FluxItem,
    rhex::{intent::Binding, payload::RhexPayload},
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
        let item = input.remove(0);
        if item.intent.schema == Binding::Bound("rhex://schema.crypto.ed25519.signed".to_string()) {
            let request = input.remove(0);
            let signature = match item.intent.data {
                RhexPayload::Binary { data } => data,
                _ => return -1,
            };

            let mut out_intent = request.intent.clone();
            out_intent.schema =
                Binding::Bound("rhex://schema.lattice.quorum.signature".to_string());
            out_intent.data = RhexPayload::Binary { data: signature };
            transform_output.push(FluxItem {
                name: request.name,
                thread: request.thread,
                availability: request.availability,
                intent: out_intent,
                correlation: None,
                meta: request.meta,
            });
        } else {
            return -1;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
