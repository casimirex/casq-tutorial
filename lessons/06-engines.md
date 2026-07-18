# Lesson 6 — Engines & scaling

**Example:** [`examples/06_engines.rs`](../examples/06_engines.rs) ·
`cargo run --example 06_engines`

## The idea

Simulating a quantum computer on a classical one is expensive — but *how*
expensive depends on the circuit. casimirQ offers several engines, and choosing
well is a real skill.

- **Statevector** — tracks all `2ⁿ` complex amplitudes. Exact and works for any
  gate, but memory doubles with each qubit: ~20 qubits is already gigabytes.
- **Clifford** — restricted to the *Clifford* gate set (`H`, `S`, `CNOT`, and
  the Paulis). It doesn't store amplitudes at all; it tracks *stabilizers*, so
  it scales to hundreds or thousands of qubits — but it can't run arbitrary
  gates (notably not `T`).
- **Auto** — let the server inspect the circuit and pick. A GHZ circuit is pure
  Clifford, so `Auto` routes it to the fast path.

The deep fact (the Gottesman–Knill theorem): circuits built only from Clifford
gates can be simulated efficiently on a classical computer. Quantum *advantage*
requires non-Clifford gates like `T`.

## The code

```rust
for engine in [Engine::Auto, Engine::Statevector, Engine::Clifford] {
    let result = client
        .run(&circuit, RunOptions::new().engine(engine).shots(2000))
        .await?;
    // same GHZ distribution from every engine
}
```

## What you'll see

All three engines return the same `|00000>`/`|11111>` split. `requested_engine`
in the response tells you what actually ran (e.g. `Auto` reports `"auto"`).

## Try it yourself

1. Add a `T` gate to the circuit and request `Engine::Clifford`. It should be
   rejected or fall back — `T` is outside the Clifford set.
2. Compare `execution_time_ms` between engines as you grow the qubit count.
3. Build a 30-qubit GHZ state on the Clifford engine. Try the same on
   statevector and watch it refuse or struggle.

## Key takeaway

Match the engine to the circuit: Clifford for large stabilizer circuits,
statevector for small general ones, `Auto` when you'd rather not think about it.

Next: [Lesson 7 — Quantum RNG](07-qrng.md)
