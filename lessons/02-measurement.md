# Lesson 2 — Measurement & probability

**Example:** [`examples/02_measurement.rs`](../examples/02_measurement.rs) ·
`cargo run --example 02_measurement`

## The idea

Quantum results are **statistical**. A single measurement of a superposition
gives one classical outcome; the interesting information — the *probability
distribution* — only emerges when you repeat the experiment many times. Each
repetition is a **shot**.

Two ways to inspect a state:

- **Exact probabilities** come from the simulator's statevector. Real hardware
  can't give you these (measuring destroys the state), but a simulator can.
- **Sampled counts** mimic real hardware: run the circuit `N` times and tally
  the outcomes. As `N → ∞`, the sample fractions approach the exact
  probabilities (the law of large numbers).

To make it concrete we prepare a *biased* qubit with a rotation. `Ry(θ)` on
`|0>` gives `P(1) = sin²(θ/2)`. With `θ = π/3`, that's exactly `0.25`.

## The code

```rust
circuit.ry(0, std::f64::consts::FRAC_PI_3); // P(1) = sin^2(pi/6) = 0.25

for shots in [10, 100, 1000, 10_000] {
    let run = client.run(&circuit, RunOptions::new().shots(shots)).await?;
    // fraction of 1s should approach 0.25
}
```

## What you'll see

The exact probabilities print as `P(|0>) = 0.75`, `P(|1>) = 0.25`. Then the
sampled estimate of `P(1)` tightens toward `0.25` as shots grow:

```
    10 shots -> P(1) ~= 0.30
   100 shots -> P(1) ~= 0.24
  1000 shots -> P(1) ~= 0.252
 10000 shots -> P(1) ~= 0.2498
```

## Try it yourself

1. Change `θ` to `π/2`. What is `sin²(π/4)`, and does the sample agree?
2. At 10 shots, run several times. How much does `P(1)` jump around? This is
   *shot noise* — it shrinks like `1/√shots`.
3. Print `run.most_probable()` and confirm it reports `|0>`.

## Key takeaway

Choosing a shot count is a real engineering trade-off: more shots mean tighter
estimates but more runtime. When you only need the exact answer and you're on a
simulator, one shot plus the statevector is enough.

Next: [Lesson 3 — Single-qubit gates](03-single-qubit-gates.md)
