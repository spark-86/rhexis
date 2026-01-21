# Scope Discovery

## Default Behavior

As a default, scopes will automatically alias `rhex://scope.name/discovery` to the scope's discovery record.

This is usually a binary composite of what is commonly requested of a scope.

## Common Values

```rust
pub struct ScopeDiscovery {
    pub ushers: Vec<Usher>,
    pub last_hash: [u8; 32],
    pub last_update: u64,

}
```

```json
{
    "ushers": [
        // usher records
    ],
    
}
```
