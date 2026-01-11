# `system.registry`

## Init

Emits the truth that `data.get` picks up and returns the blob for the registry

Descriptor:

```json
{
    "descriptor": 1,
    "name": "transform.system.registry.init",
    "version": "0.1.0",
    "requires": [],
    "interacts": [
        {
            "key": "system.registry.blob",
            "thread": "system.registry",
            "schema": "rhex://schema.system.registry.blob",
            "payload_type": "binary",
            "required_fields": null,
            "flags": ["now", "observed", "required"]
        }
    ],
    "effects": [
        {
            "key": null,
            "thread": "data.get",
            "schema": "rhex://schema.data.get",
            "payload_type": "mixed",
            "required_fields": null,
            "flags": ["now", "guaranteed",  "root"]
        },
        {
            "key": "system.registry",
            "thread": "system.registry",
            "schema": "rhex://schema.system.registry",
            "payload_type": "binary",
            "required_fields": null,
            "flags": ["soon", "guaranteed", "root"]
        }
    ],
    "bin_format": "Native"
}
```

## Load

Desciptor:

```json
{
    "descriptor": 1,
    "name": "transform.system.registry.load",
    "version": "0.1.0",
    "requires": [],
    "interacts": [
        {
            "key": null,
            "thread": "data.results",
            "schema": "rhex://schema.data.result",
            "payload_type": "mixed",
            "required_fields": null,
            "flags": ["now", "required", "anchor"] 
        },
        {
            "key": "system.registry",
            "thread": "system.registry",
            "schema": "rhex://schema.system.registry",
            "payload_type": "binary",
            "required_fields": null,
            "flags": ["soon", "required", "bind"]
        }
    ],
    "effects": [
        {
            "key": "system.registry",
            "thread": "system.registry",
            "schema": "rhex://schema.system.registry",
            "payload_type": "binary",
            "required_fields": null,
            "flags": ["now", "required", "inherit"]
        }
    ],
    "bin_format": "Native"
}
```

## Store

Descriptor:

```json
{
    "descriptor": 1,
    "name": "transform.system.registry.store",
    "version": "0.1.0",
    "requires": [],
    "interacts": [
        {
            "key": null,
            "thread": "system.registry.store",
            "schema": "rhex://schema.system.registry.store",
            "payload_type": "binary",
            "required_fields": null,
            "flags": ["now", "required", "consumed"]
        }
    ],
    "effects": [
        {
            "key": null,
            "thread": "data.put",
            "schema": "rhex://schema.data.put",
            "payload_type": "mixed",
            "required_fields": null,
            "flags": ["now", "guaranteed", "produced"]
        },
        {
            "key": null,
            "thread": "system.registry.status",
            "schema": "rhex://schema.system.registry.status",
            "payload_type": "json",
            "required_fields": null,
            "flags": ["now", "guaranteed", "produced"]
        }
    ],
    "bin_format": "Native"
}
```

## Add

- system.registry.add -> (transform.system.registry.add) -> system.registry.action

```json
{
    "descriptor": 1,
    "name": "transform.system.registry.add",
    "version": "0.1.0",
    "requires": [],
    "interacts": [
        {
            "key": null,
            "thread": "system.registry.add",
            "schema": "rhex://schema.system.registry.add",
            "payload_type": "mixed",
            "required_fields": null,
            "flags": ["now", "required", "consumed"]
        }
    ],
    "effects": [
        {
            "key": null,
            "thread": "system.registry.action",
            "schema": "rhex://schema.system.registry.actions",
            "payload_type": "mixed",
            "required_fields": null,
            "flags": ["now", "guaranteed", "produced"]
        }
    ],
    "bin_format": "Native"
}
```

## Result

- system.registry.query -> (transform.system.registry.result) -> system.registry.result

```json
{
    "descriptor": 1,
    "name": "transform.system.registry.result",
    "version": "0.1.0",
    "requires": [],
    "interacts": [
        {
            "key": null,
            "thread": "system.registry.action",
            "schema": "rhex://schema.system.registry.actions",
            "payload_type": "json",
            "required_fields": null,
            "flags": ["now", "required", "consumed"]
        },
        {
            "key": "system.registry",
            "thread": "system.registry",
            "schema": "rhex://schema.system.registry",
            "payload_type": "binary",
            "required_fields": null,
            "flags": ["soon", "required", "observed"]
        }
    ],
    "effects": [
        {
            "key": null,
            "thread": "system.registry.result",
            "schema": "rhex://schema.system.registry.result",
            "payload_type": "mixed",
            "required_fields": null,
            "flags": ["now", "guaranteed", "produced"]
        }
    ],
    "bin_format": "Native"
}
```
