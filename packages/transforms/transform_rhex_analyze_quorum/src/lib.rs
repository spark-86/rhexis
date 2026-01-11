use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        Rhex,
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
        signature::{RhexSignature, SignatureType},
    },
    transform::context::TransformContext,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct RhexAwaitingQuorum {
    pub count_needed: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();
    let quorum_signature: RhexSignature = match &input[0].intent.data {
        RhexPayload::Binary { data } => serde_cbor::from_slice(&data).unwrap(),
        _ => return -1,
    };
    let (awaiting, mut rhex) = match &input[1].intent.data {
        RhexPayload::Mixed { meta, data } => {
            let rhex: Rhex = serde_cbor::from_slice(&data[0]).unwrap();
            let awaiting: RhexAwaitingQuorum = serde_json::from_value(meta.clone()).unwrap();
            (awaiting, rhex)
        }
        _ => return -2,
    };

    let quorum_count = rhex
        .signatures
        .iter()
        .filter(|s| s.sig_type == SignatureType::Quorum)
        .count();

    if quorum_count >= awaiting.count_needed {
        // First we build the "quorum met" emission
        let mut met_intent = RhexIntent::new(RhexIntent::gen_nonce());
        met_intent.schema = Binding::Bound("rhex://schema.rhex.quorum_met".to_string());
        met_intent.data = RhexPayload::Json(json!({
            "quorum_count": quorum_count,
            "quorum_required": awaiting.count_needed,
        }));
        transform_output.push(FluxItem {
            name: format!(
                "rhex.quorum_met.{}",
                hex::encode(match rhex.intent.previous_hash {
                    Binding::Bound(b) => b,
                    _ => return -3,
                })
            ),

            thread: "rhex.process.finalize".to_string(),
            availability: FluxAvailability::Now,
            intent: met_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.rhex.analyze.quorum".to_string(),
                timestamp: 0,
            },
        });

        // And then the quorum signed rhex itself.

        rhex.signatures.push(quorum_signature);
    } else {
    }
    0
}
