# Computational Reality Engine

Ok, so this is not a normal document. This is just in case I get hit by a bus on my scooter on the way to work kind of thing. Formal documentation will come later.

## What we are building

The CRE is a concept where execution revolves around the affinity of state. If there is no state change, there is no execution.

This is not a traditional operating system or a VM/container. This is a substrate for computing, that is host agnostic that runs on top of arbitrary platforms via membranes, and treats every piece of computation as evolution of a shared structed state.

Key Pillars:

- **Flux** - the structured state field ("the universe" as CRE sees it)
- **Transforms** - the rules that change flux and call into the membrane
- **HPCs (Host Platform Capabilities)** - concrete bindings to host resources
- **Membrane** - the adapter layer that knows how to talk to the host
- **Kernel** - the orchestrator that runs transforms and keeps invariants

## Flux

Flux is the structured state respresentation of the CRE world.

Each flux item is persistant until consumed by a transform.

While only transforms can observe/consume flux items, both HPCs and transforms can _emit_ flux.

## Transforms

Transforms are the logic part of the CRE. Transforms are WASM binaries, agnostic to the system.

## HPCs (Host Platform Capabilities)

HPCs are binary single function performaces loaded into the membrane. They execute one task and one task only, and then return their results to the membrane, that then gets parsed into Flux and Kernel Directives.

## Membrane

This is the host "shim" until we get Flux native hardware. This is what corrals pre-CRE hardware, does basic things like keeping resources open.

## Kernel

This is what wrangles the transforms. The kernel examines the flux, scores the transforms and then fires them based on scores, returning the collapsed dataset back into Flux.
