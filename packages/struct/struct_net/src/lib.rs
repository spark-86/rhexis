use rhexis_core::flux::item::FluxItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetFlux {
    pub ip_addr: String,
    pub port: u16,
    pub payload: Vec<FluxItem>,
}
