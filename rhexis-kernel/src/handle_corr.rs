use rand::random;
use rhexis_core::{flux::item::FluxItem, rhp::descriptor::PatternDescriptor};

pub fn update_correlation(
    flux_item: &mut FluxItem,
    effect: &PatternDescriptor,
    parent_corr: Option<[u8; 32]>,
) -> bool {
    if effect.flags.contains(&"inherit".to_string()) && parent_corr.is_some() {
        flux_item.correlation = parent_corr.clone();
        return true;
    };
    if effect.flags.contains(&"fork".to_string()) && parent_corr.is_some() {
        flux_item.correlation = Some(blake3::hash(&parent_corr.unwrap()).as_bytes().clone());
        return true;
    };
    if effect.flags.contains(&"root".to_string()) {
        flux_item.correlation = Some(random());
        return true;
    }
    false
}
