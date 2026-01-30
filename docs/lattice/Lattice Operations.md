# Lattice Operations

## Overview

```mermaid
flowchart TB
    Start(["Stable"])
    End(["Stable"])

    DS1["lattice.scope.cache.lookup.request.{}
    lattice.scope.cache.lookup.request
    schema.lattice.scope.cache.lookup.request
    Json"]
    DS2["lattice.scope.cache.lookup.response.{}
    lattice.scope.cache.lookup.response
    schema.lattice.scope.cache.lookup.response
    Mixed"]
    DS3["net.send.flux(lattice.scope.cache.lookup.request)"]
    DS4["net.send.flux(lattice.scope.cache.lookup.response)"]
    DS5["lattice.scope.cache.table
    lattice.scope.cache.table
    schema.lattice.scope.cache.table
    Binary"]
    DS6["lattice.scope.cache.action"]
    DS7["lattice.scope.cache.error"]
    DS8["data.put"]
    DS9["system.registry"]
    DS10["system.init"]
    DS11["data.get"]
    DS12["rhex.stored"]
    DS13["net.send.flux(rhex.stored)"]
    DS14["rhex.appendable"]
    DS15["net.send.flux(error.rhex.validation.falied)"]
    DS16["rhex.final.incoming"]
    DS17["rhex.quorum.signature"]
    DS18["rhex.hash.quorum"]
    DS19["error.rhex.quorum.hash"]
    DS20["rhex.quorum.request"]
    DS21["net.send.flux(error.rhex.invalid.quorum)"]

    T1[["transform.lattice.scope.cache.lookup"]]
    T2[["transform.lattice.scope.cache.resolver"]]
    T3[["transform.lattice.scope.cache.table.update"]]
    T4[["transform.lattice.scope.cache.load"]]
    T5[["transform.lattice.rhex.append"]]
    T6[["transform.lattice.rhex.final.validator"]]
    T7[["transform.lattice.rhex.sign.quorum"]]
    T8[["transform.lattice.rhex.hash.quorum"]]

    H1[("hpc.crypto.ed25519.sign")]
    H2[("hpc.crypto.blake3.hash")]

    Start -->|Scope Lookup| DS1
    Start -->|Scope Lookup Response| DS2
    Start -->|System Startup| DS10
    Start --> DS9
    Start -->|Rhex Append Request| DS16
    Start -->|Rhex Quorum Request| DS20
    T1 -->|Found result locally| DS2
    T1 -->|Requesting| DS3
    T1 -->|Responding| DS4
    DS1 --> T1
    DS3 -->|Issue remote request| DS1
    DS4 --> DS2
    DS2 --> T2
    T2 -->|Success| DS6
    T2 -->|Error| DS7
    DS7 --> End
    DS9 --> T3
    DS6 --> T3
    T3 --> DS5
    T3 --> DS8
    DS8 --> SomeShit("Some Shit")
    DS10 --> T4
    DS9 --> T4
    T4 --> DS11
    DS16 --> T6
    T6 --> DS14
    T6 --> DS15
    DS14 --> T5
    T5 --> DS8
    T5 --> DS13
    T5 --> DS12
    DS11 --> SomeMoreShit("Some More Shit")
    DS20 --> T8
    DS5 --> T8
    SomeMoreShit --> DS5
    T8 --> H2
    T8 --> DS21
    H2 --> DS18
    DS18 --> T7
    T7 --> H1
    T7 --> DS19
    H1 --> DS17
    SomeShit --> End
    DS17 --> End
    DS5 --> End
    DS13 --> End
    DS12 --> End
    DS15 --> End
    DS19 --> End
    DS21 --> End
```

## Updating the cache

```json
{
    "scope": "...",
    "status": "found" | "parent" | "error",
    "slot0": "Table Entry"
}
```

```mermaid
flowchart TD
    Start(["Stable"])
    End(["Stable"])
    DS1["lattice.scope.cache.lookup.response.{}
    lattice.scope.cache.lookup.responses
    schema.lattice.scope.cache.lookup.response
    Mixed"]
    DS2["lattice.scope.cache.table
    lattice.scope.cache.table
    schema.lattice.scope.cache.table
    Binary"]
    DS3["lattice.scope.cache.action"]
    DS4["lattice.scope.cache.error"]
    T1[["transform.lattice.scope.cache.response.resolver"]]
    T2[["transform.lattice.scope.cache.table.update"]]
    Start --> DS1
    DS1 --> T1
    T1 -->|Successful lookup| DS3
    T1 -->|Error| DS4
    DS3 --> T2
    T2 --> DS2
    DS2 --> End
```

## Rhex Usher Submission

```mermaid
flowchart TB
    Start(["Stable"])
    End(["Stable"])

    F0["rhex.usher.sign"]
    F1["lattice.scope.cache.table"]
    F2["rhex.usher.signed (Soon)"]
    F3["crypto.ed25519.sign"]
    F4["net.send.flux(rhex.usher.signed (Now))"]
    F5["crypto.ed25519.signed"]
    F6["net.send.flux(error.rhex.usher.sign)"]
    F7@{shape: processes, label: "rhex.quorum.sign"}

    T0[["transform.lattice.rhex.usher.authorize"]]
    T1[["transform.lattice.rhex.usher.assemble"]]

    H0[("hpc.crypto.ed25519.sign")]
    H1[("hpc.net.send.flux")]

    Start -->|Incoming Flux| F0
    subgraph Authorize
    F0 --> T0
    F1 --> T0
    end
    T0 --> F6
    F6 --> H1
    T0 --> F3
    T0 --> F2
    F3 --> H0
    H0 --> F5
    subgraph Sign
    F5 --> T1
    F5 <==>|Corr Link| F2
    F2 --> T1
    end
    F1 --> T1
    T1 --> F7
    T1 --> F4
    F4 --> H1
    H1 --> End
```

## Rhex Final Submission

```mermaid
flowchart TB
    Start(["Stable"])
    End(["Stable"])

    F0["rhex.append"]
    F1["net.send.flux(rhex.submitted)"]
    F2["net.send.flux(error.rhex.append)"]
    F3["rhex.appendable"]
    F4["data.put"]
    F5["data.put.result.{}"]
    F6("...")
    F7["error.rhex.validation.failed"]

    T0[["transform.rhex.append"]]
    T1[["transform.rhex.final.validation"]]

    H1[("hpc.net.send.flux")]

    Start --> F0
    F0 --> T1
    T1 --> F3
    T1 -->|Validation Failed| F7
    F3 --> T0
    T0 --> F2
    T0 --> F1
    T0 --> F4
    F4 --> F6
    F1 --> H1
    H1 --> End
    F6 --> F5
    F5 --> End
    F2 --> H1
    F7 --> End
```
