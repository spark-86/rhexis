# Scope Lookup Breakdown

This is the process for looking up a scope's hosted location by usher.

## Process

- We consume the lattice.scope.lookup.request with transform.lattice.scope.lookup
- We check the lattice.scope.lookup.cache.table for the value. If it's not found or expired, we do a network reachout unless flags \["local"\]. Can emit lattice.lookup.cache.results or lattice.scope.lookup.remote.request
- On lattice.scope.lookup.remote.request we let transform.lattice.scope.lookup consume the request, and issue the packets to whatever network transport we use, along with a listener for completion.
- On lattice.scope.lookup.cache.results or lattice.scope.lookup.remote.results we consolidating by...
- Emitting a flux that has the results of the lookup (lattice.scope.lookup.results/rhex://schema.lattice.scope.lookup.results)

## Handlers

- `transform.lattice.scope.cache.lookup`
- `trasnform.lattice.scope.cache.update`

## RHP

### `transform.rhex.scope.cache.lookup

Descriptor

```json
{
    "descriptor": 1,
    "name": "transform.lattice.scope.cache.lookup",
    "version": "0.1.0",
    "requires": [],
    "interacts": [
        {
            "key": null,
            "thread": "lattice.scope.lookup",
            "schema": "rhex://schema.lattice.scope.lookup",
            "payload_type": "json",
            "required_fields": null,
            "flags": ["now", "consumed", "required"]
        },
        {
            "key": "lattice.scope.table",
            "thread": "lattice.scope.table",
            "schema": "rhex://schema.lattice.scope.table",
            "payload_type": "binary",
            "required_fields": null,
            "flags": ["now", "required", "observed"]
        }
    ],
    "effects": [
        {
            "key": null,
            "thread": "lattice.scope.lookup.results",
            "schema": "rhex://schema.lattice.scope.lookup.result",
            "payload_type": "json",
            "requried_fields": null,
            "flags": ["now", "optional"]
        },
        {
            "key": null,
            "thread": "lattice.scope.lookup.remote.request",
            "schema": "rhex://schema.lattice.scope.lookup.remote.request",
            "payload_type": "json",
            "required_fields": null,
            "flags": ["now", "optional"]
        }
    ],
    "bin_format": "Native"
}
```

This basically just pulls a flux out, views the HashMap stored inside, and returns a value.
