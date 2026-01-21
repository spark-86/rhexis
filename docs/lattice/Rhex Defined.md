# Rhex (R⬢) Defined

## What Is a R⬢?

A R⬢ is a cryptographically signed **intention to exist** in the lattice. Everything around it—hashes, signatures, quorum, storage—exists solely to make that intention permanent, ordered, and verifiable.

At its core, a R⬢ is the author’s declared action, bound to time, scope, and authority, then sealed into an immutable chain.

---

## Structure

```rust
pub struct Rhex {
    pub magic: [u8; 6],
    pub intent: RhexIntent,
    pub context: RhexContext,
    pub signatures: Vec<RhexSignature>,
    pub current_hash: Option<[u8; 32]>
}

pub struct RhexIntent {
    pub previous_hash: Option<[u8; 32]>,
    pub scope: Binding<String>,
    pub nonce: [u8; 32],
    pub author_public_key: Binding<[u8; 32]>,
    pub usher_public_key: Binding<[u8; 32]>,
    pub schema: Binding<String>,
    pub record_type: Binding<String>,
    pub data: RhexPayload
}

pub struct RhexContext {
    pub at: u64,
    pub spacial_ref: Option<String>,
    pub spacial_data: Option<Vec<u8>>,
}

pub struct RhexSignature {
    pub sig_type: SignatureType,
    pub public_key: [u8; 32],
    pub sig: [u8; 64]
}

pub enum RhexPayload {
    Json(serde_json::Value),
    Binary { data: Vec<u8> },
    Mixed { meta: serde_json::Value, data: Vec<Vec<u8>> },
    None,
}

pub enum Binding<T>{
    Bound(T),
    Unbound,
}
```

---

## The Intent

The **Intent** is the author’s complete declaration of what they wish to commit.

* **previous_hash** — Blake3 hash of the prior record in the scope’s chain.
* **scope** — The namespace being appended to.
* **nonce** — Random value preventing replay.
* **author_public_key** — The key asserting authorship.
* **usher_public_key** — The specific usher the author expects to perform acceptance.
* **schema** — URI of the data schema, typically `rhex://scope.name/schema.alias`.
* **record_type** — The semantic action being taken (e.g. `policy:set`, `key:grant`, `record:data`). These function like protocol-level verbs.
* **data** — Payload (≤ 1024 bytes), supporting structured JSON, binary blobs, or mixed forms.

The Intent is what is signed first. Everything else exists to validate, contextualize, and finalize it.

---

## Addressing

The `usher_public_key` binds the Intent to a specific coordinator. Once the author signs over an Intent containing this key, no other usher is permitted to accept or forward it.

This key must correspond to the public key in `signatures[1]`, the accepting usher’s signature.

---

## Signing

Signature order is canonical:

1. **Author** (`signatures[0]`)
2. **Accepting Usher** (`signatures[1]`)
3. **Quorum Members** (`signatures[2..n]`, sorted)

The author first signs the Intent.

The Intent is then sent to the addressed usher. If policy permits, the usher attaches Context and signs:

```text
H(author_signature || context_bytes)
```

This binds time and location to the author’s declaration.

---

## Quorum Collection

Each `record_type` in a scope has a quorum rule defined by its active `policy:set`, even if the quorum size is 1.

After receiving the usher’s signature and Context, the author requests signatures from quorum peers (discovered via the scope’s discovery endpoint). The author collects K valid quorum signatures, ignoring extras beyond K for liveness, though they may be retained for convergence.

If the accepting usher is also a quorum member, it may immediately provide both signatures.

---

## Finalization

Once quorum is satisfied, signatures are sorted by public-key byte order and the final record hash is computed:

```text
H(magic || intent || context || sorted_signatures)
```

This Blake3 digest becomes `current_hash`.

The completed R⬢ is resubmitted to the accepting usher, which performs:

* Signature verification
* Hash verification
* Chain continuity check (`previous_hash` == current scope `last_hash`)

If all checks pass, the usher commits the record to its storage backend, advances the scope’s hash chain, updates discovery with the new `last_hash`, and returns confirmation of permanence.

At this point, the intention has become an immutable fact of the lattice.
