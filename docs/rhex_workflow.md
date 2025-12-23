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
- Resolve storage location (local vs. remote) - transform.rhex.scope.lookup / transform.rhex.scope.manage
- Commit Rhex to local storage - transform.rhex.store
- emit rhex.submitted.{id} - transform.rhex.ready { schema.rhex.store.complete }

### Shit I think we need

- Scope look up is observing "scope.name" in the thread "rhex.scope.list" - transform.rhex.scope.lookup
