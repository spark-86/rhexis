# Usher Creation

## Init

- usher.create -> transform.lattice.usher.create -> lattice.usher.create.result (Soon) + crypto.ed25519.generate
- lattice.usher.create.result (Soon) + crypto.ed25519.generate.result -> transform.lattice.usher.scope.assign -> lattice.scope.usher.add.request
- lattice.scope.usher.add.result + lattice.usher.create.result (Soon) -> transform.lattice.usher.create.finalizer -> lattice.usher.create.final.result

## Assign to Scope

- lattice.scope.usher.add.request + lattice.scope.cache.table -> transform.lattice.scope.usher.add -> lattice.scope.usher.add.result + lattice.scope.cache.action
