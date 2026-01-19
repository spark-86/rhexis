use crate::{
    flux::item::FluxItem,
    rhex::{intent::Binding, payload::RhexPayload},
};
use base64::{Engine as _, engine::general_purpose};

pub fn pretty_print_flux(flux: &Vec<FluxItem>) {
    for item in flux {
        let previous_hash = match &item.intent.previous_hash {
            Binding::Bound(b) => format!("Bound ({})", general_purpose::URL_SAFE_NO_PAD.encode(b)),
            Binding::Unbound => format!("Unbound"),
        };
        let scope = match &item.intent.scope {
            Binding::Bound(s) => format!("Bound ({})", s),
            Binding::Unbound => format!("Unbound"),
        };
        let nonce = general_purpose::URL_SAFE_NO_PAD.encode(&item.intent.nonce);
        let author_public_key = match &item.intent.author_public_key {
            Binding::Bound(b) => format!("Bound ({})", general_purpose::URL_SAFE_NO_PAD.encode(b)),
            Binding::Unbound => format!("Unbound"),
        };
        let usher_public_key = match &item.intent.usher_public_key {
            Binding::Bound(b) => format!("Bound ({})", general_purpose::URL_SAFE_NO_PAD.encode(b)),
            Binding::Unbound => format!("Unbound"),
        };
        let schema = match &item.intent.schema {
            Binding::Bound(s) => format!("Bound ({})", s),
            Binding::Unbound => format!("Unbound"),
        };
        let record_type = match &item.intent.record_type {
            Binding::Bound(s) => format!("Bound ({})", s),
            Binding::Unbound => format!("Unbound"),
        };

        let data = match &item.intent.data {
            RhexPayload::Json(v) => format!("Json ({})", serde_json::to_string_pretty(v).unwrap()),
            RhexPayload::Binary { data } => {
                format!("Binary ({})", general_purpose::URL_SAFE_NO_PAD.encode(data))
            }
            RhexPayload::Mixed { meta, data } => {
                let mut b64view = Vec::new();
                for d in data {
                    b64view.push(general_purpose::URL_SAFE_NO_PAD.encode(d));
                }

                format!(
                    "Mixed (meta: {}, data: {:?})",
                    serde_json::to_string_pretty(meta).unwrap(),
                    b64view
                )
            }
            RhexPayload::None => format!("None"),
        };

        println!("FluxItem {{");
        println!("    name: {},", item.name);
        println!("    thread: {},", item.thread);
        println!("    availability: {:?},", item.availability);
        println!("    intent: {{");
        println!("        previous_hash: {},", previous_hash);
        println!("        scope: {},", scope);
        println!("        nonce: {},", nonce);
        println!("        author_public_key: {},", author_public_key);
        println!("        usher_public_key: {},", usher_public_key);
        println!("        schema: {},", schema);
        println!("        record_type: {},", record_type);
        println!("        data: {},", data);
        println!("    }},");
        println!("    correlation: {:?},", item.correlation);
        println!("    meta: {{");
        println!("        creator: {},", item.meta.creator);
        println!("        timestamp: {},", item.meta.timestamp);
        println!("    }},");
        println!("}}");
    }
}
