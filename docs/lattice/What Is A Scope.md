# What Is a Scope?

## More Than a Namespace

The lattice is an append‑only alignment of signed records across a measurable clock. At global scale, a single linear timeline becomes a bottleneck: contention for the same `previous_hash`, latency amplification, and coordination overhead make accurate sequencing increasingly fragile.

**Scopes** solve this by fracturing the lattice into many concurrent timelines that can later be related, cross‑validated, and federated.

On the surface, a scope is just a string composed of:

* `a–z`, `0–9`
* dot (`.`) for hierarchy
* underscore (`_`) for readability

Examples: `schema`, `schema.rhex`, `sigil`, `emotor.cert.oem`

Underneath, a scope is a *sovereign causal domain*: its own hash chain, its own policy, its own quorum, its own notion of authority. Instead of everyone appending to one global spine, writers append to many coordinated spines. Overlapping ushers and cross‑scope references create a trust‑mesh rather than a single central choke point.

The result is horizontal scalability for time itself.

---

## Scope Creation

Every scope exists under a parent. The unnamed root scope is the ultimate ancestor; top‑level scopes like `schema` and `sigil` are its children.

Creating a scope is a governed, cryptographically auditable process:

1. **Request**

   * A `scope:request` R⬢ is appended to the *parent* scope.
   * The requesting key must already have append rights in that parent.

2. **Evaluation**

   * The parent’s usher evaluates the request against its active `policy:set`.
   * Requirements may include role membership, quorum, delays, or fees.

3. **Creation**

   * If approved, the parent appends a `scope:create` R⬢.
   * This record uniquely names the child scope and anchors its lineage.

4. **Genesis**

   * Exactly one `scope:genesis` R⬢ may be appended *inside the new scope*.
   * Only the requesting key is authorized for this single write.
   * All further authority is defined by the policy embedded or referenced here.

The genesis record may immediately close the scope (by installing a policy that grants no append rights). This allows ephemeral, sealed, or historical scopes—but it also means genesis is a one‑shot operation. Treat it like launching a spacecraft: configuration errors become permanent history.

---

## Scope Policy — The Local Laws of Physics

The most powerful record in any scope is `policy:set`. It defines:

* Who may write
* What they may write
* Under which roles
* With what quorum
* At what rate
* During which time window

Policy is temporal, not static. Multiple `policy:set` records may exist; their validity is governed by micromarks.

### Effective Micromark

```rust
pub eff: u64
```

The micromark at which the policy becomes active.

* `0` means “effective immediately.”
* A future value schedules the policy to activate later, enabling deterministic governance transitions.

### Expiration Micromark

```rust
pub exp: u64
```

The micromark at which the policy ceases to be valid.

> If a policy expires and no successor is in effect, the scope becomes append‑locked. This is not a soft failure; it is a cryptographic deadbolt.

### Tags

```rust
pub tags: Vec<String>
```

Free‑form strings used for indexing, discovery, and semantic grouping. They carry no authority by themselves but are invaluable for tooling and analysis.

---

## Rules

Rules are the operational core of `policy:set`. Each rule binds permissions to record types via roles and quorum.

```rust
pub struct Rule {
    pub append_roles: Vec<String>,
    pub k: u16,
    pub quorum_roles: Vec<String>,
    pub min_delay: u64,
    pub record_types: Vec<String>,
    pub window: u64,
}
```

Field semantics:

* **append_roles**
  Roles whose keys may propose (append) the specified `record_types`.

* **k**
  Number of quorum signatures required for acceptance.

* **quorum_roles**
  Roles whose keys are authorized to provide those quorum signatures.

* **min_delay**
  Minimum micromark interval between successive appends by the same key. This is rate‑limiting at the protocol layer.

* **record_types**
  The exact set of R⬢ types governed by this rule (e.g., `policy:set`, `key:grant`, `schema:define`).

* **window**
  Maximum micromark span in which the `k` quorum signatures must be collected. After this window, the attempt expires and must be re‑proposed.

Together, these fields turn a scope into a self‑contained jurisdiction: a small universe with its own constitution, courts, clocks, and laws of causality, all enforced by hashes and time rather than by trust in any single machine or institution.
