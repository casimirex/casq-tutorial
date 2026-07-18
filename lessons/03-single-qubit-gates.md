# Lesson 3 — Single-qubit gates

**Example:** [`examples/03_single_qubit_gates.rs`](../examples/03_single_qubit_gates.rs) ·
`cargo run --example 03_single_qubit_gates`

## The idea

Any single-qubit state can be drawn as a point on the surface of a sphere (the
**Bloch sphere**): `|0>` at the north pole, `|1>` at the south, and
superpositions around the equator. Every single-qubit gate is just a **rotation**
of that sphere.

The gates you'll use constantly:

| Gate | Effect | Note |
| --- | --- | --- |
| `X` | bit flip `|0>↔|1>` | 180° about the X axis |
| `Y` | bit + phase flip | 180° about the Y axis |
| `Z` | phase flip `|1>→ -|1>` | invisible to a Z-basis measurement |
| `H` | `|0>→|+>`, `|1>→|->` | swaps Z and X axes |
| `S` | quarter phase turn | `S = √Z` |
| `T` | eighth phase turn | needed for universality |
| `Rx/Ry/Rz(θ)` | continuous rotation by θ | parameterized gates |

A phase flip like `Z` doesn't change measurement probabilities on its own — but
sandwiched between `H`s it becomes a bit flip, because `X = H·Z·H`. Phases are
invisible until interference (a later `H`) turns them into amplitude
differences. That's the engine behind most quantum algorithms.

## The code

Each row builds a one-qubit circuit and prints the resulting amplitudes:

```rust
show(&client, "H then Z then H", |c| { c.h(0).z(0).h(0); }).await?; // = X
show(&client, "S after H",       |c| { c.h(0).s(0); }).await?;      // adds i phase
show(&client, "Rx(pi/2)",        |c| { c.rx(0, FRAC_PI_2); }).await?;
```

## What you'll see

```
X (bit flip)        +1.000|1>
H (superposition)   +0.707|0>  +0.707|1>
H then Z then H     +1.000|1>            <- equals X
S after H           +0.707|0>  +0.707i|1> <- note the imaginary phase
Rx(pi/2)            +0.707|0>  -0.707i|1>
```

Watch the imaginary parts: `S` and `Rx` introduce complex phases that `H` alone
does not.

## Try it yourself

1. Confirm `Z` alone leaves `P(0)`/`P(1)` unchanged from `|0>` — phase is
   hidden without interference.
2. Build `Ry(π)` and compare it to `X`. Up to a global phase they agree.
3. Chain `T` eight times. Since `T` is a 45° phase, `T⁸` should return to start.

## Key takeaway

Single-qubit gates move one qubit around the Bloch sphere. Real power comes from
combining them with entangling gates — next.

Next: [Lesson 4 — Entanglement: Bell states](04-bell-state.md)
