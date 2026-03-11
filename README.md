# Rhexis – Computational Reality Engine

## A New Computational Ontology

Traditional programming treats **functions** as the center of computation. Functions examine state, make decisions, and mutate that state. Execution order is controlled by schedulers, event loops, or explicit calls.

The Computational Reality Engine (CRE) inverts that model.

In the CRE, **state is the primary object**. Computation occurs through **transforms**, which describe how one state becomes another. A transform does not run because it was called; it runs because the **conditions encoded in the data make it possible**.

There is no global scheduler.

Instead, transforms declare the **valence** of the data they require. When the correct state exists, the transform fires automatically and produces the next state. Each transform operates only on the data it binds to. No transform has global knowledge of the system.

The result is a system where computation emerges from **state transitions** rather than ordered execution.

## Where to Start

### Aquiring

```bash
git clone https://github.com/spark-86/rhexis
```

### Compiling

```bash
cd rhexis
cargo build
```

### Running the Current Demo

The current demo generates flux and sends it across the network to itself. This demonstrates the basic mechanics of the system:

* flux generation
* transform execution
* HPC packaging
* transport and re-ingestion of state

To build and run the demo:

Linux/MacOS:

```bash
./demo.sh
```

Windows:

```text
.\demo.cmd
```

The script will:

1. Package the required transforms and HPCs into `.rhp` files
2. Generate the initial flux state
3. Produce the configuration required to run the CRE

After packaging completes, the script pauses for confirmation. Once started, any output you see is produced by the CRE itself as it processes flux and executes transforms.

### Why It Looks Rough

It is rough.

This project explores a different model of computation, not a finished product. The current code demonstrates the mechanics of the system rather than a polished implementation.

Think of it less as a product and more as the first working prototype of a new class of engine. Early prototypes are rarely elegant—but they prove the mechanism works.
