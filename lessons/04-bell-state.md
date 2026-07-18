# Lesson 4 — Entanglement: Bell states

**Example:** [`examples/04_bell_state.rs`](../examples/04_bell_state.rs) ·
`cargo run --example 04_bell_state`

## The idea

**Entanglement** is the phenomenon that makes quantum computing more than
parallel coin flips. Two qubits are entangled when their joint state *cannot* be
written as "qubit 0 is in state A **and** qubit 1 is in state B" — the qubits
only have a definite state *together*.

The simplest example is a **Bell pair**:

```
(|00> + |11>) / √2
```

Read it carefully: the only terms are `00` and `11`. There is no `01` or `10`.
So if you measure qubit 0 and get `1`, qubit 1 is instantly guaranteed to be `1`
too — even though *which* value appears is random. The outcomes are perfectly
correlated.

You build it with two gates:

1. `H` on qubit 0 → `(|0> + |1>)/√2` on the first qubit.
2. `CNOT` (controlled-X) with control 0, target 1 → flips qubit 1 exactly when
   qubit 0 is `1`, welding the two together.

## The code

```rust
let mut bell = Circuit::new(2);
bell.h(0).cx(0, 1);
let result = client.run(&bell, RunOptions::new().shots(2000)).await?;
```

## What you'll see

```
|11>  ~1000   50%
|00>  ~1000   50%
Perfectly correlated (only 00 and 11 appear): true
```

Roughly half `00`, half `11`, and **never** `01` or `10`.

## Try it yourself

1. Add `bell.x(0)` before the `H`. You now get the `(|01> + |10>)/√2` Bell
   state — perfectly *anti*-correlated.
2. Add a `bell.z(0)` after the CNOT. The counts look identical (Z is a hidden
   phase) — this is one of the four Bell states, distinguishable only by
   interference.
3. Measure only in the counts and confirm the totals sum to your shot count.

## Key takeaway

`H` + `CNOT` is the canonical entangling pattern. Entanglement is the resource
behind teleportation, superdense coding, and the speedups in later lessons.

Next: [Lesson 5 — GHZ & multi-qubit entanglement](05-ghz-state.md)
