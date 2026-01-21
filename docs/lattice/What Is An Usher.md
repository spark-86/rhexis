# What Is an Usher?

## The Usher as a Concept

An usher is the entry point to the lattice. A lattice can exist in isolation—local keys, local storage, pure cryptography—but its purpose is to be *distributed*. Distribution requires coordination, validation, and continuity. The usher provides all three.

Conceptually, an usher is a scope-aware gatekeeper that:

* Receives proposed records (R⬢)
* Verifies them against policy and protocol rules
* Participates in quorum when required
* Commits accepted records to permanent storage
* Extends the scope’s hash chain

The name comes from the role: ushering records from intent into irreversible history.

Operationally, an usher examines an incoming R⬢ and, based on its completeness and its role within the scope’s quorum set, does one of three things:

* Signs as the **accepting append agent**
* Signs as a **quorum member**
* Rejects the submission as invalid

If the R⬢ is complete and the usher is authorized to append for the scope, it commits the record to its backing store and issues a confirmation of permanence.

---

## The Usher’s Duty to Continuity

Ushers are also the network fabric of the lattice.

Each scope exposes a discovery endpoint, conventionally aliased as:

```text
rhex://scope.name/discover
```

This resolves to the current usher peer group for that scope. Through discovery, a client learns:

* The public keys of all active appending ushers
* The current `last_hash` of the scope’s chain

This allows any participant to locate the correct quorum set and submit records without hard-coded addressing. Continuity emerges from overlapping, mutually verifying ushers rather than from any single canonical node.

---

## The Author’s Intent

The core of every R⬢ is its **Intent**: a signed declaration of what the author wishes to commit to the lattice.

When an intent is created, it is bound to an `usher_public_key`. This specifies the intended receiving usher and prevents replay or redirection across coordinators.

When the usher receives a valid intent, it constructs a **Context** and attaches it. Context includes:

* `at` — the current time in Micromarks
* `spacial_ref: String` — an optional location or reference namespace
* `spacial_data: Vec<u8>` — optional opaque spatial payload

The usher then signs over the author’s signature and the context:

```text
H(author_signature || context)
```

This produces the usher’s acceptance signature. The partially completed R⬢ is returned to the author for quorum collection. If the usher is itself a quorum member, it may also immediately add its quorum signature.

---

## Quorum: Friends and Adversaries

Quorum groups are defined by the active `policy:set` of the scope. When an usher receives a quorum request, it:

1. Verifies that its public key belongs to an authorized quorum role
2. Validates the Context (including that it falls within the allowed `window`)
3. Signs over the combined author and usher signatures:

```text
H(author_signature || usher_signature)
```

Only the signature is returned. Assembly is the author’s responsibility.

Signature ordering is deterministic:

* `Sig[0]` — Author
* `Sig[1]` — Accepting usher
* `Sig[2..n]` — Quorum signatures, sorted lexicographically by public key bytes

This canonical ordering ensures that all nodes compute identical hashes for identical records.

---

## Final Submission

Once all required signatures are collected and ordered, the final record hash is computed:

```text
H(magic || intent || context || sorted_signatures)
```

This Blake3 digest becomes `current_hash`.

The fully assembled R⬢ is then resubmitted to the original usher for finalization. The usher performs:

* Signature verification (author, usher, quorum)
* Hash verification (`current_hash`)
* Chain continuity check (`previous_hash` == scope `last_hash`)

If all checks pass, the usher commits the record to its storage backend, advances the scope’s temporal chain, and publishes the new `last_hash` for discovery.

At that point, the intent has crossed the event horizon: it is no longer a proposal, but an immutable fact of the lattice.

## The Usher Data Structure

```rust
pub struct Usher {
    pub name: String,
    pub public_key: [u8; 32],
    pub priority: u8,
    pub location: UsherLocation,
    pub last_updated: u64,
}

pub enum UsherLocation {
    Local,
    Remote { ip_addr: String, port: u16 },
}
```
