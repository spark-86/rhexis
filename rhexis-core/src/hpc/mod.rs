pub mod context;
pub mod descriptor;
pub mod directive;
pub mod entry;
pub mod errors;
pub mod hpc_return;

pub trait HighPerformanceCapability {
    fn capability_id(&self) -> &'static str;
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, String>;
}
