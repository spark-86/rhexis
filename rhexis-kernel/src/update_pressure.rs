use super::Kernel;

impl Kernel {
    pub fn update_pressure(&mut self) {
        for thread in self.flux_pond.keys() {
            let working_thread = self.flux_pond.get(thread).unwrap();
            let mut pressure = 0;
            for schema in working_thread.keys() {
                let bucket = working_thread.get(schema).unwrap();
                if bucket.len() == 64 {
                    pressure += 1;
                }
            }
            if pressure == 0 {
                self.thread_pressure.remove(thread);
                continue;
            } else {
                self.thread_pressure.insert(thread.clone(), pressure);
            }
        }
    }
}
