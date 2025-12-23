# The bullshit I need to write down so I can actually understand how this is gonna work

So this is like one of the first transform/hpc combos I wrote and now coming at it from the right way this is not at all written right. The abstractional truth is we have to build in queuing because we don't want to saturate the disk.

## How this has to work, I think

So we have to first harvest all of thread:data.put/schema:rhex://schema.data.put flux (up to 64) items via `transform.data.put`. This transform will evaluate them to see what medium we are gonna use for each one, and then emit them to the appropriate thread to be handled.

Then we handle each thread as single grouping, in `transform.data.put.{target}.batch`, process it in batches to be outputted to `data.put.{target}.outbox` (prolly 8 HPC fires per item)

`transform.data.put.{target}.fire` consumes exactly one from `data.put.{target}.outbox` per cycle, executing up to 8 writes per medium.

## What I think we need

- hpc.data.put.disk
- transform.data.put
- transform.data.put.disk.batch
- transform.data.put.disk.fire

## Data shapes

For the actual data structure going into the HPC @ transform.data.put.disk.fire

Descriptor:

```json
{
    "descriptor": 1,
    "name": "transform.data.put.disk.fire",
    "version": "0.1.0",
    "requires": ["data.put.disk"],
    "interacts": [
        {
            "key": null,
            "thread": "data.put.disk.queue",
            "schema": "rhex://schema.data.put.disk.batch",
            "payload_type": "Binary",
            "required_fields": null,
            "flags": ["consumed", "now", "required"]
        }
    ],
    "effects": [
        {       
            "key": null,
            "thread": "data.put.history",
            "schema": "rhex://schema.data.put.history",
            "payload_type": "Json",
            "required_fields": null,
            "flags": ["soon", "produced"]
        }
    ],
    "bin_format": "Native"
}
```

```rust
pub struct DataPayload {
    pub logical_id: [u8; 32],
    pub data: Vec<u8>
}
```

For the actual data structure going into `transform.data.put.disk.fire`

```text
thread: data.put.disk.queue
schema: rhex://schema.data.put.disk.queue
```

```rust
pub struct DataBatch {
    payloads: Vec<DataPayload>,
}
```

For the actual data structure going into `transform.data.put.disk.batch`

This is just a match in the descriptor for the transforms:

```json
{
    "thread": "data.put.disk",
    "schema": "rhex://schema.data.put.disk"
}
```

Which is just again `DataPayload` again.

It get's a little spicer at the router, with the constraints coming into play to help route. We have a RhexPayload::Mixed type of:

```json
{
    "constraints": [
        "local",
        "disk",
        "ram",
        "remote",
        "lattice"
    ]
}
```

With the data being:

```rust
data[0] = logical_id;
data[1] = data_payload;
```
