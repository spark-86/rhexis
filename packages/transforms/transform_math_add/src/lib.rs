use rhexis_core::{
    flux::{item::FluxItem, meta::FluxMeta, payload::FluxPayload},
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde_json::json;

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let input = unsafe { &(*ctx).input };

    let left = match &input[0].payload {
        FluxPayload::Json(v) => v,
        _ => return 1,
    };
    let right = match &input[1].payload {
        FluxPayload::Json(v) => v,
        _ => return 1,
    };

    let left_num = match left.as_f64() {
        Some(v) => v,
        None => return 1,
    };
    let right_num = match right.as_f64() {
        Some(v) => v,
        None => return 1,
    };

    let sum = left_num + right_num;

    unsafe {
        (*ctx).output.push(FluxItem {
            name: "math.result".to_string(),
            schema: None,
            payload: rhexis_core::flux::payload::FluxPayload::Json(json!({"value": sum})),
            meta: FluxMeta {
                creator: "transform.math.add".to_string(),
                timestamp: 0,
            },
        });
    };

    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
