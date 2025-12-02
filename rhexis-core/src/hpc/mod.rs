use crate::membrane::probe::EnvironmentProbe;

pub mod context;
pub mod descriptor;
pub mod entry;
pub mod errors;

pub trait HighPerformanceCapability {
    fn capability_id(&self) -> &'static str;
    fn can_activate(&self, env: &EnvironmentProbe) -> bool;
    fn init(&mut self, env: &EnvironmentProbe);
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, String>;
}
