# Lesson 23 — Transpilation

**Example:** [`examples/23_transpilation.rs`](../examples/23_transpilation.rs) ·
`cargo run --example 23_transpilation`

## The idea

Lesson 21 noted that a backend has a **native gate set** — the gates it runs
directly — and reported how far a circuit was from it. This lesson does the
actual work: **transpilation**, rewriting a circuit into that native basis.

Real hardware doesn't run "a Hadamard" or "a Toffoli". It runs a small fixed
set of physical operations. Everything else must be **decomposed** into that
set. The decomposition is *exact* — the transpiled circuit computes the same
thing — but it almost always uses **more gates**. That gate-count growth is one
of the central challenges of near-term quantum computing: more gates means more
noise (Lesson 20), and coherence runs out (Lesson 18).

casimirQ transpiles to the basis `{rz, ry, cx}`:

- Any single-qubit gate → a few `rz`/`ry` rotations (a general Euler
  decomposition).
- Any *singly-controlled* gate — `cx`, `cy`, `cz`, `ch`, `cp`, `crx`/`cry`/`crz`,
  or an arbitrary controlled-U — decomposes into two `cx` plus rotations (the
  standard "ABC" identity). `cx` and `cz` keep hand-optimized fast paths.
- `swap` and the Toffoli decompose into `cx` + rotations.

Controlled-**phase** matters especially: it's the building block of the Quantum
Fourier Transform (Lesson 10). Because the transpiler decomposes `cp`, a full
QFT comes back **fully native** — ready for hardware.

## The code

```rust
let mut bell = Circuit::new(2);
bell.h(0).cx(0, 1);

let t = client.transpile(&bell).await?;
println!("{} -> {} gates, native: {}", t.original_gate_count, t.transpiled_gate_count, t.fully_native);

// The result is a runnable native circuit.
let native = t.to_circuit(2);
let run = client.run(&native, RunOptions::new().shots(2000)).await?;
```

## What you'll see

```
Bell state:
  2 gates -> 3 native gates ["id", "rz", "ry", "cx"]
  native ops: ["rz", "ry", "cx"]        // the Hadamard became rz, ry
  measured (still |00>/|11> only): ...   // exact — still a Bell state

Toffoli circuit:
  3 gates -> 23 native gates {cx: 6, ry: 4, rz: 13}

3-qubit QFT:
  7 gates -> 24 native gates {cx: 9, ry: 3, rz: 12}
  fully native: true   (controlled-phase decomposed to rz/ry/cx)
```

The Bell circuit's Hadamard becomes two rotations, and the transpiled circuit
**still measures as a Bell state** — the rewrite preserved the computation. The
Toffoli, a single 3-qubit gate, explodes into ~20 native operations. And the QFT
— built from controlled-phase rotations — comes back **fully native**: every
`cp` was decomposed into `cx` + rotations. That is the real cost hardware pays
for a "simple" gate.

## Try it yourself

1. Transpile a GHZ preset. How does the native gate count scale with the number
   of qubits?
2. Build a larger QFT (4–5 qubits) and transpile it. Count the `cx` gates — this
   is exactly the two-qubit-gate budget that limits QFT depth on real hardware.
3. Transpile a circuit with a `cswap` (Fredkin) gate. It still comes back
   `fully_native: false` with `cswap` in `unsupported` — controlled-SWAP and
   multi-controlled gates beyond the Toffoli are the transpiler's next frontier.
4. Compare `transpiled_gate_count` for a circuit of Hadamards vs the same number
   of `cx` gates. Which "costs" more to run natively, and why?

## Key takeaway

Transpilation is the bridge between the circuit you *write* and the gates a
device can *run*. It's exact but not free: the gate-count blow-up it exposes is
exactly what makes deep circuits hard on real, noisy hardware — and why
shallow, hardware-aware algorithm design matters.

Back to the [overview](../README.md).
