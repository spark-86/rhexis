use rhexis_core::{
    flux::{item::FluxItem, payload::PayloadType},
    rhex::{intent::Binding, payload::RhexPayload},
    rhp::descriptor::PatternDescriptor,
};

pub fn check_effects(
    effects: Vec<PatternDescriptor>,
    flux_item: FluxItem,
) -> Option<PatternDescriptor> {
    let mut success = false;
    let mut fail_strs = Vec::new();
    for effect in effects {
        let mut score = 0;
        let mut required = 0;
        if effect.key.is_some() {
            required += 1;
            if effect.key == Some(flux_item.name.clone()) {
                score += 1;
            } else {
                fail_strs.push("KEY".to_string());
            }
        }
        if effect.schema.is_some() {
            required += 1;
            match &flux_item.intent.schema {
                Binding::Bound(v) => {
                    if effect.schema == Some(v.to_string()) {
                        score += 1;
                    } else {
                        fail_strs.push(format!("SCHEMA:{:?}", flux_item.intent.schema).to_string());
                    }
                }
                Binding::Unbound => {
                    fail_strs.push("SCHEMA".to_string());
                }
            };
        }
        // Add one to required for payload_type and then auto add 1
        // if we hit "Any"
        required += 1;
        if effect.payload_type == PayloadType::Any {
            score += 1;
        } else {
            let start = score;
            match &flux_item.intent.data {
                RhexPayload::Binary { data: _ } => {
                    if effect.payload_type == PayloadType::Binary {
                        score += 1;
                    }
                }
                RhexPayload::Json(_) => {
                    if effect.payload_type == PayloadType::Json {
                        score += 1;
                    }
                }
                RhexPayload::Mixed { meta: _, data: _ } => {
                    if effect.payload_type == PayloadType::Mixed {
                        score += 1;
                    }
                }
                RhexPayload::None => {
                    if effect.payload_type == PayloadType::None {
                        score += 1;
                    }
                }
            }
            if score == start {
                fail_strs.push("PAYLOAD_TYPE".to_string());
            }
        }

        if score == required {
            success = true;
        } else {
            println!(
                "{:?} failed to output, failed with {:?}, scored {}/{}",
                effect, fail_strs, score, required
            );
        }
        if success {
            return Some(effect);
        }
    }
    None
}
