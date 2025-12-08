use rhexis_core::flux::{item::FluxItem, json::JsonFluxItem};
use serde_json::Value;

pub fn parse_flux_json(flux_json: Value) -> Vec<FluxItem> {
    let mut flux_state = Vec::new();
    for item in flux_json.as_array().unwrap() {
        let json_flux: JsonFluxItem = serde_json::from_value(item.clone()).unwrap();
        let flux_item = FluxItem::from_json(&serde_json::to_string(&json_flux).unwrap());
        flux_state.push(flux_item);
    }
    flux_state
}
