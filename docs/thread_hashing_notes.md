# Thread-Scoped Hashing for Incremental Transform Evaluation

## Abstract

This paper describes the design, implementation, and advantages of **thread-scoped hashing** as an optimization mechanism for a state-driven, emergent computation engine. The system operates on a shared world-state (the *flux pond*) partitioned by *threads* and *schemas*, and executes *transforms* reactively based on declared visibility and patterns.

Rather than rescoring all transforms on every cycle, thread-scoped hashing enables **incremental reactivity**: only transforms whose visible state has changed are considered for scoring and execution. This technique preserves full correctness, replayability, and determinism while dramatically reducing per-cycle overhead as the number of registered transforms grows.

The paper presents the underlying data model, the motivation for thread hashing, a concrete implementation strategy, and the performance and architectural advantages that emerge from this approach.

---

## 1. Background and Motivation

### 1.1 The State-Driven Execution Model

The computation model assumes:

* A global, append-and-prune state called the **flux pond**
* State elements (*flux items*) grouped by **thread** and **schema**
* Declarative **transform descriptors** that describe what state a transform can observe and what state it may emit
* A discrete execution loop composed of *cycles*

Each cycle consists of:

1. Ingesting new flux
2. Scoring transforms against visible state
3. Executing eligible transforms
4. Consuming, detonating, and materializing state
5. Repeating until convergence

This model emphasizes **presence and structure over control flow**. Transforms do not run because they are scheduled; they run because the world has entered a state that satisfies their declared patterns.

### 1.2 The Scaling Concern

In early implementations, transform scoring involved iterating over the full transform registry every cycle. While acceptable for small systems, this approach becomes problematic as the system scales toward thousands of transforms.

Crucially, however, most transforms are *latent* in any given cycle. They observe only a small subset of threads, and most threads do not change on most cycles.

This observation motivates the central question:

> Why rescore transforms whose observable world has not changed?

Thread-scoped hashing provides a principled answer.

---

## 2. The Flux Pond Data Model

### 2.1 Structural Partitioning

The flux pond is modeled as a nested map:

```text
Thread → Schema → [FluxItem]
```

Where:

* **Thread** represents a locality or lifecycle domain
* **Schema** represents the structural shape of data
* **FluxItem** is an immutable fact with optional correlation

This partitioning is not an optimization; it is the semantic foundation of the system. All visibility, scoring, and execution decisions are made relative to this structure.

### 2.2 Deterministic Hashing

To detect convergence and ensure replayability, the system computes a **global hash** of the flux pond each cycle. This hash is computed deterministically by:

1. Sorting threads
2. Sorting schemas within each thread
3. Sorting flux items within each schema bucket
4. Hashing their serialized representations

Thread-scoped hashing extends this idea without altering its semantics.

---

## 3. From Global Hashing to Thread Hashing

### 3.1 The Key Insight

The key insight is:

> If a transform only observes a specific set of threads, and none of those threads have changed, then the transform’s score cannot change.

This is true because the system enforces the following invariants:

* Transforms declare all threads they observe (via descriptors)
* Scoring is pure (no side effects)
* Execution is gated (at most once per transform per cycle)
* State mutation occurs only through materialization / consumption / detonation

These constraints allow the kernel to reason about *change locality*.

### 3.2 Per-Thread Hashes

Instead of computing only a global hash, the kernel computes:

```text
thread_hashes: Map<ThreadId, Hash>
```

Each thread hash represents the complete state of that thread, independent of other threads.

The global hash can still be derived as a deterministic combination of the sorted thread hashes, preserving convergence semantics.

---

## 4. Implementation Strategy

### 4.1 Hash Computation

For each cycle:

1. Iterate over all threads
2. Compute a deterministic hash of that thread’s schema buckets and flux items
3. Store the result in `thread_hashes`
4. Compare against `prev_thread_hashes`

Threads whose hashes differ are marked as **changed**.

### 4.2 Transform Visibility Indexing

At transform registration time, build an index:

```text
thread → [transform_id]
```

Each transform registers the threads it may observe based on its descriptor’s `interacts` clauses and any `bind` clause.

This index is static and does not change at runtime.

### 4.3 Candidate Selection

At the start of each cycle:

1. Determine the set of changed threads
2. Union the transform lists associated with those threads
3. Score only those transforms

Transforms that observe multiple threads are rescored if **any** of their visible threads change. This conservative approach preserves correctness.

---

## 5. Correctness and Determinism

### 5.1 Preservation of Semantics

Thread hashing does not alter:

* scoring rules
* execution ordering
* consumption semantics
* detonation behavior

It only avoids unnecessary recomputation.

If a transform is skipped, it is because its observable world is provably identical to the previous cycle.

### 5.2 Replayability

Because thread hashes are derived from deterministic serialization and sorted traversal, replaying the same input sequence produces identical per-thread and global hashes.

This ensures:

* bit-for-bit reproducibility
* stable convergence detection
* auditable execution

---

## 6. Performance Characteristics

### 6.1 Asymptotic Behavior

Let:

* **T** = total number of registered transforms
* **V** = average number of transforms visible per thread
* **C** = number of threads changed in a cycle

Without thread hashing:

```text
O(T) scoring per cycle
```

With thread hashing:

```text
O(C × V) scoring per cycle
```

In typical workloads, **C is small** and **V ≪ T**, yielding major reductions.

### 6.2 Desktop-Scale Implications

This makes scenarios such as:

* 10,000 registered transforms
* 100–200 threads
* 3–5 threads changing per cycle

entirely practical on modern desktop hardware.

---

## 7. Architectural Advantages

### 7.1 Locality of Reasoning

Thread hashing reinforces a core design principle:

> State changes are local, and computation should be too.

Developers can reason about performance and behavior by inspecting which threads are active and which transforms depend on them.

### 7.2 Compatibility With Future Optimizations

Thread hashes naturally support:

* cycle skipping (no-work cycles)
* partial replay
* debug diffs between cycles
* thread-level profiling and heat maps

All without modifying transform logic.

---

## 8. Relationship to Other Systems

Thread-scoped hashing is related in spirit to:

* incremental build systems (only rebuild what changed)
* reactive UI frameworks (re-render only what changed)
* spreadsheets (recalculate only dependent cells)

However, it differs in a crucial way:

* dependencies are **semantic and declarative**, not inferred dynamically

This reduces invalidation bugs and makes behavior auditable.

---

## 9. Limitations and Trade-offs

Thread hashing introduces:

* additional memory for storing per-thread hashes
  n- per-cycle overhead to compute those hashes
* conservative rescoring when transforms observe many threads

These costs are predictable and bounded, and they are typically dwarfed by the scoring work avoided.

---

## 10. Conclusion

Thread-scoped hashing transforms the kernel from a brute-force reactive loop into an **incremental, locality-aware system**.

By leveraging explicit visibility, deterministic state structure, and immutable facts, the engine avoids unnecessary work while preserving all semantic guarantees.

This optimization is not a shortcut; it is a natural consequence of designing the system around structure, causality, and observability from the start.

In practice, thread hashing provides a clear bridge between expressiveness and scalability, enabling large transform ecosystems to operate efficiently in a desktop-class environment.

---
