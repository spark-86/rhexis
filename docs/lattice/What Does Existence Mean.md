# What Does Existence Mean in the Lattice?

## Identity

The minimal unit of identity in the lattice is an Ed25519 keypair. Possession of the private key and the ability to produce valid signatures is the sole primitive required to assert authorship and continuity. Everything else—names, roles, reputation, and authority—is layered on top of this cryptographic root.

---

## SigilID

A **SigilID** is the human-readable, short-form identifier derived from an identity’s public key.

It is computed as the Crockford Base32 encoding of the first 80 bits of the public key and stored internally as an unformatted string. For display and convention, it is typically rendered in quad form:

```text
XXXX-XXXX-XXXX-XXXX
```

SigilIDs are not intended to be globally unique forever; they are compact, memorable anchors. In the extremely rare event of collision, a new keypair is generated, as the predecessor cannot be overwritten.

---

## VeroScore

To participate meaningfully in public regions of the lattice, an identity accrues a **VeroScore**.

* New identities begin at `0`.
* Self-identification, third-party attestations, and independent verification increase the score.
* The scale ranges from `0.0` to `100.0` (decimal).

VeroScore is not a moral judgment. It is a measure of *legitimacy and confidence*, used primarily for:

* Spam resistance
* Sybil mitigation
* Access gating in public scopes

A high score means “many independent parties converge on this identity being real and persistent,” nothing more.

---

## Anchoring Identity in the Lattice

To exist durably, an identity anchors itself by claiming its SigilID as a scope.

For a SigilID `XXXX-XXXX-XXXX-XXXX`, the canonical anchor scope is:

```text
sigil.XXXX-XXXX-XXXX-XXXX
```

Creation proceeds as follows:

1. The keypair is generated.
2. A `scope:request` R⬢ is submitted to the parent `sigil` scope, signed by that key.
3. The request asks for creation of `sigil.XXXX-XXXX-XXXX-XXXX`.
4. The `sigil` ushers verify that the requesting public key derives to the claimed SigilID.
5. If valid, they must authorize and issue `scope:create`.
6. The requester may then append exactly one `scope:genesis` inside the new scope.

The `sigil` scope’s policy allows open creation, rate-limited to one lifetime request per SigilID. This guarantees that every valid key has a permanent, globally discoverable home in the lattice.

Permanence is not implicit; it is claimed. If you can generate a key and form a minimal CBOR R⬢, you can write something that will persist as long as the lattice itself persists.

---

## Attestations

Trust is built through **attestations**: signed statements by one identity about another.

The conventional location for attestations about a SigilID is:

```text
sigil.XXXX-XXXX-XXXX-XXXX.attestation
```

A clean governance pattern is:

1. An external party requests permission to attest.
2. The identity temporarily updates policy to allow a single append.
3. The attestation R⬢ is written.
4. The scope is immediately closed again.

This preserves an identity-centric, append-minimal history while still allowing:

* Individual attestations
* Batched attestations for group events
* Aggregated proofs in single canonical records

---

## Key Preservation

Keys are fragile. Humans lose them.

For any identity of lasting value, private keys should be protected using Shamir Secret Sharing and distributed as shards to trusted custodians. Each shard is stored offline. Reconstruction requires a threshold of participants, allowing recovery without any single person holding the full secret.

This is not optional hygiene; it is existential continuity.

---

## README

By convention, most Sigil scopes expose a default human-facing document:

```text
rhex://sigil.XXXX-XXXX-XXXX-XXXX/readme
```

This is an alias to the most recent `record:markdown` designated as the identity’s primary presentation. Global Sigil browsers resolve and display this by default, making it the entry point for understanding a given identity.

---

## Roles

Cryptographic identity establishes *who*. Roles establish *what authority that who has* within a scope.

Roles are simple, unique strings.

When a scope is created:

* The author of `scope:genesis` is automatically assigned the `keymaster` role.
* `keymaster` may:

  * Install the first `policy:set`
  * Grant roles to keys
  * Delegate governance

Role assignment is performed with `key:grant` R⬢ records, which bind a public key to one or more roles within a scope. Revocation is performed with `key:revoke`, which removes all roles for that key in that scope; continued participation then requires explicit re-grant.

Roles, combined with policy and quorum, turn raw keys into structured, governable identities inside the lattice.
