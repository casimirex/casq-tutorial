# Lesson 9 — Grover's search

**Example:** [`examples/09_grover.rs`](../examples/09_grover.rs) ·
`cargo run --example 09_grover`

## The idea

You have an unstructured collection of `N = 2ⁿ` items and a way to recognize the
one you want (an oracle that flags it). Classically you must, on average, look at
`N/2` items. **Grover's algorithm** finds it in about `√N` steps — a *quadratic*
speedup that applies to a huge range of brute-force search problems.

Grover works by **amplitude amplification**, repeating two moves:

1. **Oracle** — flip the phase of the marked item (mark it with a minus sign).
2. **Diffusion** — reflect all amplitudes about their average, which nudges the
   marked amplitude up and everything else down.

Each iteration rotates the state a little closer to the marked item. The magic
number is `≈ (π/4)·√N` iterations: too few and the marked item isn't yet likely;
*too many* and you rotate past it and the success probability drops again.

## The code

casq-sdk exposes Grover through the algorithms API, which assembles the oracle
and diffusion circuit for you:

```rust
let result = client.algorithms().grover(n, marked, None).await?; // None = optimal iters
println!("{:.4}", result.success_probability);
```

Passing `None` for the iteration count uses the optimal `≈ (π/4)·√N`.

## What you'll see

```
  n   N=2^n   optimal iters   success probability
  2       4               1   1.0000
  3       8               2   0.9453
  4      16               3   0.9613
  5      32               4   0.9992
  6      64               6   0.9966
```

A handful of iterations makes the marked item overwhelmingly likely.

## Try it yourself

1. Force too many iterations, e.g. `grover(4, 9, Some(10))`. Watch the success
   probability fall — over-rotation is real.
2. Change the marked item; the success probability shouldn't depend on *which*
   item is marked, only on `N`.
3. Grover gives quadratic, not exponential, speedup. For `N = 1,000,000`, how
   many iterations is `√N`?

## Key takeaway

Grover is the canonical example of a broad quantum speedup for search. The
oracle/diffusion loop is a reusable primitive in many algorithms.

Next: [Lesson 10 — Quantum Fourier Transform](10-qft.md)
