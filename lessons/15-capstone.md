# Lesson 15 — Capstone: build your first quantum app

**Example:** [`examples/15_capstone.rs`](../examples/15_capstone.rs) ·
`cargo run --example 15_capstone`

## The idea

You've learned the physics and the algorithms. Now put on your engineer hat and
build a small **application** that uses the whole stack the way a real service
would. A quantum app is rarely "one circuit" — it's circuits *plus* persistence,
sampling, and higher-level algorithm calls, wired together with ordinary code.

This capstone walks the full `casq-sdk` surface:

1. **Build** a parameterized circuit (an entangled register with a phase
   rotation).
2. **Persist** it on the server and **list** your saved circuits back.
3. **Run the stored circuit** by id — the save/reuse pattern real apps need.
4. **Draw a quantum-random byte** (Lesson 7 as a reusable component).
5. **Call a pre-built algorithm** (Grover) alongside your own circuits.
6. **Clean up** the circuit you created.

## The code

```rust
// 2. persist
let record = client.create_circuit("capstone-demo", &circuit).await?;
// 3. run the stored circuit by id
let sim = client.run_stored(&record.id, RunOptions::new().shots(2000)).await?;
// 5. mix in a pre-built algorithm
let grover = client.algorithms().grover(4, 9, None).await?;
// 6. tidy up
client.delete_circuit(&record.id).await?;
```

Notice how naturally the quantum calls sit inside plain async Rust — error
handling with `?`, loops, functions. That's the point: a quantum backend is just
another service your program talks to.

## What you'll see

The program saves a circuit, lists your circuits, runs the stored one (a GHZ-like
50/50 split), prints a random byte, reports Grover's success probability, and
deletes the circuit — a complete round trip.

## Where to go next

- **Design your own circuit** and expose it behind a small CLI or HTTP service.
- **Combine primitives:** e.g. use the QRNG to seed randomized inputs to an
  algorithm.
- **Explore the SDK:** `create_circuit`, `list_circuits`, `get_circuit`,
  `run`, `run_stored`, and the full `algorithms()` surface (`qft`, `grover`,
  `shor`, `teleport`, `vqe`, `qaoa`).
- **Go deeper on theory:** error correction, quantum error mitigation, and the
  broader map of quantum complexity are the natural next frontier.

## Key takeaway

Real quantum software is classical software that calls quantum operations at the
right moments. You now have the vocabulary, the primitives, and a working
end-to-end app to build on.

Congratulations — you've gone from a single qubit to a complete quantum
application. Back to the [overview](../README.md).
