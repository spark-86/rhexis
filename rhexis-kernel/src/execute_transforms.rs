use std::collections::HashMap;

use crate::{ExecutionArtifacts, ScoreResult};

use super::Kernel;

impl Kernel {
    pub fn execute_transforms(&self, scores: HashMap<String, ScoreResult>) -> ExecutionArtifacts {
        let mut artifacts = ExecutionArtifacts {
            collapse_map: HashMap::new(),
            consumed: Vec::new(),
            detonators: Vec::new(),
            hpc_calls: Vec::new(),
            diag: Vec::new(),
        };

        for (id, score) in scores {
            let transform = self.transform_registry.get(&id).unwrap();

            artifacts.consumed.extend(score.consumed.iter().cloned());

            self.execute_single(transform, score, &mut artifacts);
        }

        artifacts
    }
}
