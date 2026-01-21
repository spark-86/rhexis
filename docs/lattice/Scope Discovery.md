# Scope Discovery

## Default Behavior

As a default, scopes will automatically alias `rhex://scope.name/discovery` to the scope's discovery record.

This is usually a binary composite of what is commonly requested of a scope.

## Common Values

`rhex://schema.lattice.scope.discovery/@0` dictates the following structure:

```rust
pub struct ScopeDiscovery {
    pub ushers: Vec<Usher>,
    pub last_hash: [u8; 32],
    pub last_update: u64,
}
```

See [What Is An Usher?](<What Is An Usher.md>) for the usher structure

## Sample Scope Discovery

```rust
ScopeDiscovery {
    ushers: [
        Usher {
            name: "Usher Alice".to_string(),
            public_key: [ 0, 0, 0, 0, 0, 0, ... ],
            priority: 10,
            location: UsherLocation::Remote{
                ip_addr: "1.2.3.4".to_string(),
                port: 1984
            }
            last_updated: 1234,
        },
        Usher {
            name: "Usher Bob".to_string(),
            public_key: [ 1, 1, 1, 1, 1, 1, ... ],
            priorty: 20,
            location: UsherLocation::Local,
            last_updated: 2345,
        },
    ],
    last_hash: [ 1, 2, 3, 4, 5, 6, 7, ... ],
    last_updated: 3456,
}
```

## Path To Scope Discovery Retrieval

- transform.lattice.scope.discover -> lattice.get

- net.flux -> transform.net.process -> lattice.response
- lattice.response -> transform.lattice.response.process -> lattice.* (literally not bound to schema)
- lattice.scope.discovery -> transform.lattice.scope.discovery.update -> lattice.scope.cache.update
