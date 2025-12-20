use std::fmt::Write;

use super::Kernel;

impl Kernel {
    pub fn dump_flux_tree(&self) -> String {
        let mut out = String::new();
        let pond = &self.flux_pond;

        writeln!(out, "Flux Pond").unwrap();

        // ---- sort threads ----
        let mut threads: Vec<_> = pond.keys().collect();
        threads.sort();

        for (t_idx, thread) in threads.iter().enumerate() {
            let thread_prefix = if t_idx + 1 == threads.len() {
                "└─"
            } else {
                "├─"
            };
            writeln!(out, "{} thread: {}", thread_prefix, thread).unwrap();

            let schema_map = &pond[*thread];

            // ---- sort schemas ----
            let mut schemas: Vec<_> = schema_map.keys().collect();
            schemas.sort();

            for (s_idx, schema) in schemas.iter().enumerate() {
                let schema_prefix = if t_idx + 1 == threads.len() {
                    if s_idx + 1 == schemas.len() {
                        "   └─"
                    } else {
                        "   ├─"
                    }
                } else {
                    if s_idx + 1 == schemas.len() {
                        "│  └─"
                    } else {
                        "│  ├─"
                    }
                };

                writeln!(out, "{} schema: {}", schema_prefix, schema).unwrap();

                let bucket = &schema_map[*schema];

                for (_f_idx, flux) in bucket.iter().enumerate() {
                    let flux_prefix = if t_idx + 1 == threads.len() {
                        if s_idx + 1 == schemas.len() {
                            "      └─"
                        } else {
                            "      ├─"
                        }
                    } else {
                        if s_idx + 1 == schemas.len() {
                            "│     └─"
                        } else {
                            "│     ├─"
                        }
                    };

                    write!(out, "{} {}", flux_prefix, flux.name).unwrap();

                    // correlation (compact)
                    if let Some(c) = flux.correlation {
                        write!(
                            out,
                            "  corr={:02x}{:02x}{:02x}{:02x}…",
                            c[0], c[1], c[2], c[3]
                        )
                        .unwrap();
                    } else {
                        write!(out, "  (no corr)").unwrap();
                    }

                    // optional: payload hint
                    //write!(out, "  [{:?}]", flux.intent.schema).unwrap();

                    writeln!(out).unwrap();
                }
            }
        }

        out
    }
}
