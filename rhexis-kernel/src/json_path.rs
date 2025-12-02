use serde_json::Value;

pub trait JsonPathExt {
    fn contains_path(&self, path: &str) -> bool;
}

impl JsonPathExt for Value {
    fn contains_path(&self, path: &str) -> bool {
        let mut cur = self;

        for seg in path.split('.') {
            // Support array indexes like "items[0].id"
            let (key, idx) = if let Some(open) = seg.find('[') {
                let close = seg.rfind(']').unwrap_or(seg.len());
                let key = &seg[..open];
                let idx_str = &seg[open + 1..close];
                (key, idx_str.parse::<usize>().ok())
            } else {
                (seg, None)
            };

            // Step into object key
            cur = match (cur, key) {
                (Value::Object(map), k) => match map.get(k) {
                    Some(next) => next,
                    None => return false,
                },
                _ => return false,
            };

            // Optional array index
            if let Some(i) = idx {
                cur = match cur {
                    Value::Array(a) => a.get(i).unwrap_or(&Value::Null),
                    _ => return false,
                };
                if cur.is_null() {
                    return false;
                }
            }
        }

        true
    }
}
