# Communication in the Computaional Reality Engine

## Flux As The Universal Shape

Flux are designed to be the ultimate atomic truth. Either be it for local computation, or remote emission, the flux makes a perfect solution.

## Net.Flux

Flux alone handles most data states, it just lacks addressing. Net.Flux, as defined in `rhex://schema.net.flux` is an outer wrapper around a Vec of flux.

```rust
pub struct NetFlux {
    sig: [u8; 64],
    key: [u8; 32],
    ip_addr: String,
    port: u16,
    payload: Vec<FluxItem>
    gt: u64
}
```

Hash is the Blake3 over the whole thing.

```text
H(ip_addr || port || payload || gt)
```

This prevents replay simply by keeping a chart of received hashes. We should never receive more than the same hash once ever.

These can then be routed and batched as needed. e.g. packing a heavy load to a single usher in one connection. Connection streams expect and transmit `Vec<NetFlux>`

This ironically is the imperfect and lossy way of doing things.

## Just Stick It In The Lattice

The other method of communication that lies at the heart of the lattice itself, the Rhex. You can actually just store the Flux into the lattice as a Rhex and have listener transforms waiting on the other end.

This allows the authenticity to be verified, as well as proving permanence.
