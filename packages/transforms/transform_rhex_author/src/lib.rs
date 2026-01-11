use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        Rhex,
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde_json::json;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    for item in input {
        let (meta, rhex) = match item.intent.data {
            RhexPayload::Mixed { meta, data } => {
                (meta, serde_cbor::from_slice::<Rhex>(&data[0]).unwrap())
            }
            _ => return -1,
        };

        // Validate
        if !rhex.intent.scope.is_bound() {
            return -2;
        }
        if !rhex.intent.record_type.is_bound() {
            return -3;
        }
        // Hash intent
        let hash = rhex.calc_author_hash().unwrap();

        let mut waiting_intent = RhexIntent::new(RhexIntent::gen_nonce());
        waiting_intent.schema = Binding::Bound("rhex://schema.rhex.author.waiting".to_string());
        waiting_intent.data = RhexPayload::Mixed {
            meta: meta.clone(),
            data: vec![serde_cbor::to_vec(&rhex).unwrap()],
        };

        let mut author_request = RhexIntent::new(RhexIntent::gen_nonce());
        author_request.schema = Binding::Bound("rhex://schema.rhex.author.request".to_string());
        author_request.data = RhexPayload::Mixed {
            meta,
            data: vec![serde_cbor::to_vec(&hash).unwrap()],
        };

        let target_scope = match rhex.intent.scope {
            Binding::Bound(scope) => scope,
            _ => return -4,
        };
        let mut lattice_request = RhexIntent::new(RhexIntent::gen_nonce());
        lattice_request.schema =
            Binding::Bound("rhex://schema.lattice.scope.cache.request".to_string());
        lattice_request.data = RhexPayload::Json(json!({
            "scope": target_scope
        }));

        transform_output.push(FluxItem {
            name: format!("rhex.author.waiting.{}", hex::encode(&rhex.intent.nonce)),
            thread: "rhex.author.waiting".to_string(),
            availability: FluxAvailability::Eventually,
            intent: waiting_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.rhex.author".to_string(),
                timestamp: 0,
            },
        });
        transform_output.push(FluxItem {
            name: format!("rhex.author.request.{}", hex::encode(&rhex.intent.nonce)),
            thread: "rhex.author.request".to_string(),
            availability: FluxAvailability::Now,
            intent: author_request,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.rhex.author".to_string(),
                timestamp: 0,
            },
        });
        transform_output.push(FluxItem {
            name: format!(
                "lattice.scope.cache.request.{}",
                hex::encode(&rhex.intent.nonce)
            ),
            thread: "lattice.scope.cache.requests".to_string(),
            availability: FluxAvailability::Now,
            intent: lattice_request,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.rhex.author".to_string(),
                timestamp: 0,
            },
        });
    }

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
