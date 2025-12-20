use rhexis_core::{flux::item::FluxItem, rhex::intent::RhexIntent};
use serde_json::Value;

use crate::build::FluxBuildSpec;

pub fn parse_flux_json(flux_json: Value) -> Vec<FluxItem> {
    let mut flux_state = Vec::new();

    for item in flux_json.as_array().unwrap() {
        let json_flux: FluxBuildSpec = serde_json::from_value(item.clone()).unwrap();
        let correlation = json_flux.correlation.clone();

        let mut intent = RhexIntent::new(RhexIntent::gen_nonce());
        let data = json_flux.intent.data.materialize().unwrap();
        intent.schema = json_flux.intent.schema;
        intent.data = data;

        let flux_item = FluxItem {
            name: json_flux.name,
            thread: json_flux.thread,
            availability: json_flux.availability,
            intent,
            correlation,
            meta: json_flux.meta,
        };

        flux_state.push(flux_item);
    }

    flux_state
}
