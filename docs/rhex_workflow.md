# Local Rhex Workflow

## Done = Awaiting next action

This means we have processed everything that could need recording to our local copy of the lattice.

## How we get there

- There's no Rhex needing composing
- There's no Rhex that need signing
- There's no Rhex that need quorum (we're assuming our local usher is the quorum/submission point)
- There's no Rhex that need to be appended to the local storage

## Workflows

### Create new Rhex entries

- Monitor thread "rhex.build" for items with schema: rhex://schema.rhex.presigned - transform.rhex.build
- Validate Rhex structure (we have to make sure the payload is valid, not over 1k, etc), if there's a resolvable issue it emits "rhex://schema.rhex.waiting" while whatever resolver runs. - transform.rhex.validate
- Payloads over 1k go down the IPFS route once we build it.
- Sign w/ the author key from the enclave/HPC - transform.rhex.author_sign
- Decide if we host this scope (currently yes to all because no net)
- Stamp GT on the record & Sign w/ the usher key from the enclave/HPC - transform.rhex.usher_sign
- Sign w/ same usher key for quorum - transform.rhex.quorum_sign
- Calculate current_hash - transform.rhex.finalize
- Resolve storage location (local vs. remote) - transform.lattice.scope.cache.lookup/transform.lattice.scope.cache.add/update/remove
- Commit Rhex to local storage - transform.rhex.store
- emit rhex.submitted.{id} - transform.rhex.ready { schema.rhex.store.complete }

### Shit I think we need

## `transform.rhex.signature.arrival`

Descriptor:

```json
{
    "descriptor": 1,
    "name": "transform.rhex.signature.arrival",
    "version": "0.1.0",
    "requires": [],
    "interacts": [
        {
            "key": null,
            "thread": "rhex.signatures",
            "schema": "rhex://schema.rhex.signature",
            "payload_type": "binary",
            "flags": ["now", "required", "consumed"]
        }
    ],
    "effects": [
        {
            "key": null,
            "thread": "rhex.quorum",
            "schema": "rhex://schema.rhex.quorum.signature",
            "payload_type": "binary",
            "flags": ["now", "optional", "inherit"]
        }
    ],
    "bin_format": "Native"
}
```

## `transform.rhex.signature.quorum.validate

Descriptor:

```json
{
    "descriptor": 1,
    "name": "transform.rhex.signature.quorum.validate",
    "version": "0.1.0",
    "requires": [],
    "interacts": [
        {
            "key": null,
            "thread": "rhex.quorum",
            "schema": "rhex://schema.rhex.quorum.signature",
            "payload_type": "binary",
            "required_fields": null,
            "flags": ["now", "consumed", "required"]
        }
    ],
    "bind": {
        "thread": "rhex.quorum",
        "schema": "rhex://schema.rhex.awaiting_quorum"
    },
    "effects": [
        {
            "key": null,
            "thread": "rhex.quorum",
            "schema": "rhex://schema.rhex.quorum.valid.signature",
            "payload_type": "binary",
            "required_fields": null,
            "flags": ["now", "optional", "inherit"]
        }
    ],
    "bin_format": "Native"
}
```

## `transform.rhex.quorum_add`

This transform's whole job is to check each validated signature to make sure it gets attached to the right Rhex as well as emitting "quorum met".

Descriptor:

```json
{
    "descriptor": 1,
    "name": "transform.rhex.quorum_add",
    "version": "0.1.0",
    "requires": [],
    "interacts": [
        {
            "key": null,
            "thread": "rhex.quorum",
            "schema": "rhex://schema.rhex.quorum.valid.signature",
            "payload_type": "binary",
            "requried_fields": null,
            "flags": ["now", "consumed", "requried"]
        },
    ],
    "bind": {
        "thread": "rhex.quorum",
        "schema": "rhex://schema.rhex.awaiting_quorum"
    },
    "effects": [
        {
            "key": null,
            "thread": "rhex.quorum",
            "schema": "rhex://schema.rhex.awaiting_quorum",
            "payload_type": "mixed",
            "required_fields": null,
            "flags": ["now", "optional", "inherit"]
        },
        {
            "key": null,
            "thread": "rhex.quorum",
            "schema": "rhex://schema.rhex.quorum_met",
            "payload_type": "mixed",
            "required_fields": null,
            "flags": ["now", "optional", "inherit"]
        }
    ],
    "bin_format": "Native"
}
```

## `transform.rhex.analyze.quorum`

Descriptor:

```json
{
    "descriptor": 1,
    "name": "transform.rhex.analyze.quorum",
    "version": "0.1.0",
    "requires": [],
    "interacts": [
        {
            "key": null,
            "thread": "rhex.quorum",
            "schema": "rhex://schema.rhex.quorum.signature",
            "payload_type": "Json",
            "required_fields": null,
            "flags": ["now", "required", "consumed"]
        }
    ],
    "bind": {
        "thread": "rhex.quorum",
        "schema": "rhex://schema.rhex.awaiting_quorum"
    },
    "effects": [
        {
            "key": null,
            "thread": "rhex.quorum",
            "schema": "rhex://schema.rhex.awaiting_quorum",
            "payload_type": "Mixed",
            "required_fields": null,
            "flags": ["soon", "optional", "inherit"],
        },
        {
            "key": null,
            "thread": "rhex.process.finalize",
            "schema": "rhex://schema.rhex.quorum_met",
            "payload_type": "Json",
            "required_fields": null,
            "flags": ["now", "optional", "inherit",]
        },
        {
            "key": null,
            "thread": "rhex.process.finalize",
            "schema": "rhex://schema.rhex.finalize",
            "payload_type": "Mixed",
            "required_fields": null,
            "flags": ["now", "optional", "inherit"]
        }
    ]
}
```

## `transform.rhex.finalize`

Descriptor:

```json
{
    "descriptor": 1,
    "name": "transform.rhex.finalize",
    "version": "0.1.0",
    "requires": [],
    "interacts": [
        {
            "key": null,
            "thread": "rhex.process.finalize",
            "schema": "rhex://schema.rhex.quorum_met",
            "payload_type": "mixed",
            "required_fields": null,
            "flags": ["now", "consumed", "required"]
        },
    ],
    "bind": {
        "thread": "rhex.process.finalize",
        "schema": "rhex://schema.rhex.finalize"
    },
    "effects": [
        {
            "key": null,
            "thread": "rhex",
            "schema": "rhex://schema.rhex.store",
            "payload_type": "mixed",
            "required_fields": null,
            "flags": ["now", "optional", "inherit"]
        }
    ],
    "bin_format": "Native"
}
```

## Data paths

- Author Intent Data -> Author composes R⬢ -> rhex.intent/rhex://schema.rhex.build
- -> We query for the scope's ushers -> lattice.scope.cache.lookup/rhex://schema.lattice.scope.cache.lookup
  - -> Is the scope cached? If so -> lattice.scope.cache.results/rhex://schema.lattice.scope.cache.result
  - -> If the scope is not cached -> lattice.scope.remote.request/rhex://schema.lattice.scope.remote.request
    - -> System returns a network packet with the selection -> lattice.scope.cache.results/rhex://schema.lattice.scope.cache.result
- -> Returns an array of ushers that we pick from at weighted random -> (rhex.usher.selection/rhex://schema.rhex.usher.selection + rhex.usher.list/rhex://schema.rhex.usher.list)
- (rhex.usher/rhex://schema.rhex.build + *rhex.usher.selection/rhex://schema.rhex.usher.selection) -> Signs the R⬢ -> rhex.submit.quorum/rhex://schema.rhex.submit.quorum
- (rhex.submit.quorum/rhex://schema.rhex.submit.quorum + rhex.usher.list/rhex://schema.rhex.usher.list) -> Build quorum requests -> (rhex.quorum.awaiting_quorum + rhex.quorum.requests/rhex://schema.rhex.quorum.request)
  - -> If our usher is local, go ahead and sign -> rhex.quorum.signatures/rhex://schema.rhex.quorum_signature
  - -> If not local, we must send the request out -> rhex.submit.quorum.out/rhex://schema.rhex.quorum.out
    - -> On return rhex.quorum.signatures/rhex://schema.rhex.quorum_signature
- rhex.quorum/rhex://schema.rhex.awaiting_quorum
- rhex.quorum/rhex://schema.rhex.quorum_met
- rhex.finalize/rhex://schema.rhex.finalize
- rhex.store/rhex://schema.rhex.store
- rhex.commit/rhex://schema.rhex.commit

## Transform Cards

### TRANSFORM: `lattice.quorum.instantiate`

Submits the Rhex for quorum validation.

FACTS PRESENT:

- rhex.submit.quorum/rhex://schema.rhex.submit.quorum
- lattice.ushers/rhex://schema.lattice.usher.group

CONSUMES:

- rhex.submit.quorum/rhex://schema.rhex.submit.quorum
- lattice.ushers/rhex://schema.lattice.usher.group

EMITS:

- rhex.quorum/rhex://schema.rhex.awaiting_quorum
- lattice.quorum.request/rhex://schema.lattice.quorum.request

INVARIANTS:

- Does no logical assesment here. Leave it up to lattice.quorum.process_requests.

### TRANSFORM: `transform.rhex.quorum.advance`

Takes the output from either the local resolver or the remote and adds the signature to the pond @ rhex.quorum?

FACTS PRESENT:

- lattice.quorum.signature
- rhex.awaiting_quorum

CONSUMES:

- lattice.quorum.signature
- rhex.awaiting_quorum

EMITS:

- rhex.awaiting_quorum
- rhex.quorum_met

### TRANSFORM: `rhex.quorum.add`

Adds a validated signature from a remote usher for quorum verification

FACTS PRESENT:

- rhex.quorum/rhex://schema.rhex.awaiting_quorum
- rhex.quorum/rhex://schema.rhex.quorum.valid.signature

CONSUMES:

- rhex.quorum/rhex://schema.rhex.awaiting_quorum
- rhex.quorum/rhex://schema.rhex.quorum.valid.signature (bound)

EMITS:

- rhex.quorum/rhex://schema.rhex.awaiting_quorum
- rhex.quorum/rhex://schema.rhex.quorum_met
  
INVARIANTS:

- Checks after each new signature if we have met quorum
  
### TRANSFORM: `rhex.finalize`

Calculates the final hash over the Rhex and then emits what is necessary to store it

FACTS PRESENT:

- rhex.finalize/rhex://schema.rhex.finalize

CONSUMES:

- rhex.finalize/rhex://schema.rhex.finalize

EMITS:

- rhex.store/rhex://schema.rhex.store

INVARIANTS:

- This assumes we have already validated signatures... no further validation is done in this transform.

### TRANSFORM: `rhex.store`

Takes the completed Rhex and finds the permanent home for the it

FACTS PRESENT:

- rhex.store/rhex://schema.rhex.store

CONSUMES:

- rhex.store/rhex://schema.rhex.store

EMITS:

- rhex.store.remote/rhex://schema.rhex.store.remote
- data.put/rhex://schema.data.put
- rhex.commit/rhex://schema.rhex.commit

INVARIANTS:

- If local we submit to data.put, otherwise we are emitting to rhex.store.remote

## States

- rhex.json -> (transform.rhex.import.json) -> rhex.intent
- rhex.intent -> (transform.rhex.author) ->  rhex.author.request / rhex.author.waiting / lattice.scope.cache.lookup
- lattice.scope.cache.result / rhex.author.awaiting -> (transform.author.sign) -> rhex.author.request / rhex.author.awaiting
- rhex.author.signature / rhex.author.awaiting -> (transform.rhex.usher.submit) -> rhex.usher.request / rhex.usher.awaiting
- rhex.usher.signature / rhex.usher.awaiting -> (transform.rhex.quorum.request) -> lattice.quorum.request / rhex.quorum.waiting
- lattice.quorum.signature / rhex.awaiting_quorum -> (transform.rhex.quorum.advance) -> rhex.quorum.awaiting / rhex.finalize
- rhex.finalize -> (transform.rhex.finalize) -> rhex.store.request
- rhex.store.complete -> (transform.rhex.summarize) -> ui.output

Rhex Store

- rhex.store.request -> (transform.rhex.store) -> data.put / lattice.put / rhex.store.status
- data.put.complete / lattice.put.result / rhex.store.status -> (transform.rhex.store.confirm) -> rhex.store.complete

Lattice Signature

- lattice.quorum.request -> (transform.lattice.quorum.request.signatures) lattice.quorum.local.signature.request / lattice.quorum.remote.signature.request
- lattice.quorum.local.signature.request -> (transform.lattice.quorum.local.sign) -> crypto.ed25519.sign
- crypto.ed25519.sign -> (transform.crypto.ed25519.sign) -> crypto.ed25519.signed
- crypto.ed25519.signed / lattice.quorum.local.signature -> (transform.lattice.quorum.local.signature.collector) -> lattice.quorum.signature

- lattice.inbound -> lattice.inbound.signature
- lattice.inbound.signature -> lattice.quorum.signature

Lattice Scope Lookup

- lattice.scope.cache.lookup -> (transform.lattice.scope.cache.lookup) -> lattice.scope.cache.result / lattice.scope.remote.request

Lattice Local Scope Cache Load

- lattice.init -> (transform.lattice.initializer) -> data.get / lattice.scope.cache.table (Availability:Soon)
- data.object / lattice.scope.cache.table -> (transform.lattice.scope.cache.load) -> lattice.scope.cache.table
