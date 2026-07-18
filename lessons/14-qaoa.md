# Lesson 14 — Quantum Approximate Optimization Algorithm (QAOA)

**Example:** [`examples/14_qaoa.rs`](../examples/14_qaoa.rs) ·
`cargo run --example 14_qaoa`

## The idea

QAOA is VQE's sibling, aimed at **combinatorial optimization** — problems where
you search a huge space of discrete choices for the best one. The classic
benchmark is **MaxCut**: split a graph's vertices into two groups so that the
number of edges crossing between groups is as large as possible. MaxCut is
NP-hard, so we chase good *approximate* answers.

Like VQE, QAOA is variational. It alternates two kinds of layers:

- a **cost** layer that phases states according to how good their cut is, and
- a **mixer** layer that spreads amplitude between candidate solutions,

each with its own tunable angle. A classical optimizer tunes the angles to push
the measured cut value up. The number of layer-pairs is `p`: larger `p` can
reach better solutions but deepens the circuit (and demands more from noisy
hardware). It's the same hybrid quantum/classical loop as VQE, applied to
optimization instead of chemistry.

## The code

```rust
let graphs = client.algorithms().qaoa_examples().await?; // name -> graph
let g = &graphs["square"];
let r = client.algorithms().qaoa(g.n, &g.edges, Some(1)).await?; // p = 1
println!("best cut = {}", r.best_cut_value);
```

A graph is just a vertex count `n` and a list of edges `(u, v)`.

## What you'll see

```
  K4       (4 vertices, 6 edges): best cut = 4  (expectation 3.000)
  square   (4 vertices, 4 edges): best cut = 4  (expectation 3.000)
  triangle (3 vertices, 3 edges): best cut = 2  (expectation 2.000)
```

For a 4-cycle ("square"), the optimal cut is 4 (alternate the vertices) — QAOA
finds it. A triangle can cut at most 2 of its 3 edges.

## Try it yourself

1. Define your own graph — a 5-cycle, say — and run QAOA on it. What's the best
   possible cut by hand, and does QAOA match?
2. Increase `p`. Does the expectation value improve toward the true optimum?
3. `best_cut_value` is the best measured; `max_expectation` is the *average*
   quality. Why is the average lower?

## Key takeaway

QAOA maps optimization onto a tunable quantum circuit. It's one of the most
studied candidates for near-term quantum usefulness — and a natural fit for
routing, scheduling, and portfolio-style problems.

Next: [Lesson 15 — Capstone](15-capstone.md)
