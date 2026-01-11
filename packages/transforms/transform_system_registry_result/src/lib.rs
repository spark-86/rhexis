use rhexis_core::{
    flux::{availability::FluxAvailability, item::FluxItem, meta::FluxMeta},
    rhex::{
        intent::{Binding, RhexIntent},
        payload::RhexPayload,
    },
    transform::{context::TransformContext, entry::TransformEntry},
};
use serde_json::json;
use struct_registry::{Registry, RegistryEntry};

#[unsafe(no_mangle)]
pub extern "C" fn transform_entry(ctx: *mut TransformContext) -> i32 {
    let ctx = unsafe { &mut *ctx };
    let mut input: Vec<FluxItem> = serde_cbor::from_slice(&ctx.input).unwrap();
    let mut transform_output: Vec<FluxItem> = Vec::new();

    let table_flux = input.remove(0);
    let mut table: Registry = match &table_flux.intent.data {
        RhexPayload::Binary { data } => serde_cbor::from_slice::<Registry>(&data).unwrap(),
        _ => return -1,
    };

    for item in input {
        let (meta, bin) = match &item.intent.data {
            RhexPayload::Mixed { meta, data } => (meta, data),
            _ => return -1,
        };

        let result: Result<(), anyhow::Error> = match meta["action"].as_str().unwrap() {
            "add" => {
                let entry: RegistryEntry = serde_cbor::from_slice(&bin[0]).unwrap();
                table.insert(meta["entry_name"].to_string(), entry);
                Ok(())
            }

            "update" => {
                let updated_item: RegistryEntry = serde_cbor::from_slice(&bin[0]).unwrap();
                let entry = table.get_mut(&meta["entry_name"].to_string()).unwrap();
                *entry = updated_item;

                Ok(())
            }
            "remove" => {
                table.remove(&meta["entry_name"].to_string());
                Ok(())
            }
            _ => {
                return -1;
            }
        };
        let result_text = if result.is_ok() { "success" } else { "error" };
        let mut result_intent = RhexIntent::new(RhexIntent::gen_nonce());
        result_intent.schema = Binding::Bound("rhex://schema.system.registry.result".to_string());
        result_intent.data = RhexPayload::Mixed {
            meta: json!({
                "action": meta["action"],
                "result": result_text,
            }),
            data: bin.to_vec(),
        };

        transform_output.push(FluxItem {
            name: format!(
                "system.registry.result.{}",
                hex::encode(&result_intent.nonce)
            ),
            thread: "system.registry.results".to_string(),
            availability: FluxAvailability::Now,
            intent: result_intent,
            correlation: None,
            meta: FluxMeta {
                creator: "transform.system.registry.result".to_string(),
                timestamp: 0,
            },
        })
    }

    let mut table_intent = RhexIntent::new(RhexIntent::gen_nonce());
    table_intent.schema = Binding::Bound("rhex://schema.system.registry".to_string());
    table_intent.data = RhexPayload::Binary {
        data: serde_cbor::to_vec(&table).unwrap(),
    };

    transform_output.push(FluxItem {
        name: "system.registry".to_string(),
        thread: "system.registry".to_string(),
        availability: FluxAvailability::Now,
        intent: table_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.system.registry.result".to_string(),
            timestamp: 0,
        },
    });

    let mut store_intent = RhexIntent::new(RhexIntent::gen_nonce());
    store_intent.schema = Binding::Bound("rhex://schema.system.registry.store".to_string());
    store_intent.data = RhexPayload::None;

    transform_output.push(FluxItem {
        name: "system.registry.store".to_string(),
        thread: "system.registry.store".to_string(),
        availability: FluxAvailability::Now,
        intent: store_intent,
        correlation: None,
        meta: FluxMeta {
            creator: "transform.system.registry.result".to_string(),
            timestamp: 0,
        },
    });

    *ctx.output = Some(serde_cbor::to_vec(&transform_output).unwrap());
    0
}

#[unsafe(no_mangle)]
pub static RHEX_TRANSFORM: TransformEntry = TransformEntry {
    entry: transform_entry,
};
