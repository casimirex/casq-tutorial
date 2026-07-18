# Lesson 5 — GHZ & multi-qubit entanglement

**Example:** [`examples/05_ghz_state.rs`](../examples/05_ghz_state.rs) ·
`cargo run --example 05_ghz_state`

## The idea

Entanglement scales. Extend the Bell recipe to `n` qubits and you get the
**GHZ state** (Greenberger–Horne–Zeilinger):

```
(|00…0> + |11…1>) / √2
```

Every qubit is entangled with every other: measure any one and *all* of them
collapse to the same value together. GHZ states show up in error correction,
quantum networking, and tests of quantum mechanics itself.

The construction is a Hadamard followed by a **ladder of CNOTs** that copies the
superposition outward:

```
H(0), CNOT(0→1), CNOT(1→2), …, CNOT(n-2→n-1)
```

## The code

```rust
fn ghz(n: usize) -> Circuit {
    let mut c = Circuit::new(n);
    c.h(0);
    for q in 0..n - 1 {
        c.cx(q, q + 1);
    }
    c
}
```

Factoring circuit construction into a function like this is how you build larger
programs: parameterize by qubit count and compose.

## What you'll see

For `n = 3, 4, 5` you get only the all-zeros and all-ones strings, ~50/50:

```
GHZ state on 4 qubits:
|1111|  ~1000  50%
|0000|  ~1000  50%
Only all-0 and all-1 states appear: true
```

## Try it yourself

1. Change the ladder to fan out from qubit 0 instead: `CNOT(0→1), CNOT(0→2), …`.
   You get the same GHZ state — there's more than one circuit for it.
2. Insert an `X` on the middle qubit before the ladder. How does the output
   change?
3. Build GHZ on 12 qubits. It still runs fast here because it's a *Clifford*
   circuit — which is exactly the subject of the next lesson.

## Key takeaway

Complex entangled states are built by composing simple gate patterns in loops.
The GHZ ladder is a reusable building block.

Next: [Lesson 6 — Engines & scaling](06-engines.md)
