# Lesson 1 — Hello, Quantum

**Example:** [`examples/01_hello_quantum.rs`](../examples/01_hello_quantum.rs) ·
`cargo run --example 01_hello_quantum`

## The idea

A classical bit is either 0 or 1. A **qubit** can be in a *superposition* of
both at once, written

```
|ψ> = α|0> + β|1>
```

where `α` and `β` are complex numbers called amplitudes. You never observe the
amplitudes directly — when you **measure**, you get `0` with probability `|α|²`
and `1` with probability `|β|²`, and those must sum to 1.

Every qubit starts in `|0>`. The **Hadamard gate** (`H`) is the workhorse that
creates superposition. Applied to `|0>` it produces

```
H|0> = (|0> + |1>) / √2
```

an equal blend, so a measurement is a perfect 50/50 coin flip.

## The code

```rust
let mut circuit = Circuit::new(1); // one qubit, starts in |0>
circuit.h(0);                      // put it into superposition

let result = client
    .run(&circuit, RunOptions::new().engine(Engine::Statevector).shots(1000))
    .await?;
```

`shots(1000)` measures the prepared state 1000 independent times. We print both
the **sampled counts** and the **exact statevector** the simulator computed.

## What you'll see

```
|0>  ~500   50%
|1>  ~500   50%

|0>: amplitude +0.7071  (probability 0.500)
|1>: amplitude +0.7071  (probability 0.500)
```

`0.7071 ≈ 1/√2` — those are the amplitudes of `H|0>`. The counts hover around
50/50, drifting a little each run because measurement is genuinely random.

## Try it yourself

1. Remove `circuit.h(0)`. What distribution do you get, and why?
2. Apply `H` **twice** (`circuit.h(0).h(0)`). Predict the result before running —
   `H` is its own inverse, so `H·H = I`.
3. Bump `shots` to 100000. Do the counts get closer to exactly 50/50?

## Key takeaway

Superposition + measurement is the foundation of everything that follows. A
quantum program *prepares* a state; measurement *samples* it.

Next: [Lesson 2 — Measurement & probability](02-measurement.md)
