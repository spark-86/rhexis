pub struct EnvironmentProbe;

impl EnvironmentProbe {
    pub fn new() -> Self {
        Self
    }

    pub fn get_environment(&self) -> String {
        "dev".to_string()
    }
}
