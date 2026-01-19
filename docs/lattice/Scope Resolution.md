# Scope Resolution and How It Occurs

This document describes how a client resolves a scope, discovers the authorities responsible for it, validates policy, and ultimately submits an R⬢ (Rhex record) into the lattice with quorum-backed finality.

---

## Finding the First Available Ancestor

Scopes form a strict hierarchy using dot-separated names, built from root toward leaf. A scope such as `schema.rhex.intent` therefore has the parent `schema.rhex`, which in turn has the parent `schema`, which ultimately descends from the root (the empty scope).

Resolution always begins with the fully-qualified requested scope. The client first checks its local cache for a previously-resolved scope descriptor. A descriptor minimally contains:

* The scope name
* Its parent scope
* Known ushers for read and write
* Known quorum members
* Hash of the current scope policy
* Expiration / refresh hints

If an exact match is not present or is stale, the resolver walks *up* the hierarchy, stripping one segment at a time:

* Try `schema.rhex.intent`
* If unknown, try `schema.rhex`
* If unknown, try `schema`
* If unknown, fall back to the root scope

The root (empty) scope is assumed to be universally known and pinned at bootstrap. Every node verifies the root descriptor on first network contact, making it the cryptographic and topological anchor for all other scopes.

This upward walk continues until a scope is found for which the client possesses a valid, non-expired descriptor signed by a trusted usher or quorum.

---

### Inside the CRE

What does this look like in the CRE? A request is sent to lattice.scope.cache.requests. If the scope isn't local, the CRE attempts to resolve it by issuing a claim in lattice.scope.remote.requests.

---

## Walking Back Down

Once a known ancestor is located, resolution proceeds *downward* toward the requested leaf scope.

For each child step, the client queries the parent’s read ushers for the child’s scope descriptor. Each response is:

* Signature-verified
* Hash-checked against any previously-seen version
* Compared for quorum consistency if multiple answers differ

This continues iteratively:

* From `schema` → `schema.rhex`
* From `schema.rhex` → `schema.rhex.intent`

At each level, the resulting descriptor provides:

* The authoritative writing ushers
* The quorum ushers
* The scope policy hash
* Any delegation or inheritance rules

By the time the leaf is reached, the client holds a full, signed chain of scope ancestry proving that the target scope exists and which authorities govern it.

---

## Getting Scope Policy

With the leaf scope descriptor in hand, the client selects one of the read ushers (weighted by advertised capacity and trust score) and requests the full scope policy document.

The policy defines, at minimum:

* Accepted R⬢ record types
* Required fields and schemas
* Key roles permitted to write
* Quorum parameters (N, K, time windows)
* Size limits and rate limits
* Any semantic constraints (e.g. monotonic counters, uniqueness rules)

Although policy could theoretically be skipped and discovered only through rejection, retrieving it first prevents wasting quorum bandwidth on records that are guaranteed to fail.

The client performs local sanity checks:

* The author key is permitted for the intended record type
* The intent schema matches policy
* The record size is within limits
* Any required fields or signatures are present

Only after these checks does the record proceed to submission.

---

## Selecting the Submission Usher

From the scope descriptor, the client selects a writing usher using weighted random choice. Weights incorporate:

* Advertised throughput
* Historical responsiveness
* Trust score
* Topological proximity

The client then constructs the R⬢ Intent:

* Inserts the selected usher’s public key as the target
* Includes the scope, record type, and payload hash
* Signs the Intent with the author’s private key

The unsigned Context is left empty at this stage.

The R⬢ is transmitted to the chosen writing usher. The usher:

1. Verifies the author signature
2. Verifies policy compliance
3. Assigns a timestamp in micromarks (`at`)
4. Optionally attaches spatial or jurisdictional metadata
5. Signs over: `Intent || Context || AuthorSignature`

The result is returned to the author as a partially-complete R⬢ containing both the author and primary usher signatures.

---

## Going for Quorum

The record must now obtain quorum attestations.

From the scope descriptor, the client selects quorum ushers (again by weight). If the quorum is defined as K-of-N:

* Ideally, the client sends the signed record to all N
* Practically, it may send to a larger subset and race responses
* The first K valid signatures are sufficient

Each quorum usher independently:

1. Verifies author and writing-usher signatures
2. Confirms the timestamp is within the allowed window
3. Re-evaluates scope policy
4. Ensures no conflicting record has already been finalized

If valid, the usher signs over:

`AuthorSignature || WritingUsherSignature || Context`

These signatures are returned to the client and appended to the R⬢.

---

## Submission

Once at least K quorum signatures are collected, the client computes `current_hash` over the entire canonical R⬢ structure, including all signatures.

The fully-formed record is resubmitted to a writing usher. That usher:

* Re-verifies all quorum signatures
* Verifies the final hash
* Checks for race or duplication

Upon success, the record is:

* Written to the usher’s permanent storage
* Linked into the local lattice
* Broadcast to mirror peers

At this point, the record is logically committed, though not yet fully replicated.

---

## Mirroring and Completeness

All mirror nodes for the scope begin replicating the new R⬢ immediately.

Each mirror:

* Verifies the full signature chain
* Verifies the hash
* Stores the record immutably
* Attaches it to its local lattice segment

When at least K of the N mirrors acknowledge durable storage, the record is considered *complete*.

Completion means:

* The record is now a permanent fact
* It cannot be removed, only superseded by new records
* Any future scope resolution will eventually converge on this state

At this stage the R⬢ becomes an undestroyable entity in the computational reality: a signed, quorum-anchored, permanently addressable fragment of truth.
