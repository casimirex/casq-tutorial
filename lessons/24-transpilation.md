# Lesson 24 — Transpilation

**Example:** [`examples/24_transpilation.rs`](../examples/24_transpilation.rs) ·
`cargo run --example 24_transpilation`

## The idea

Lesson 22 noted that a backend has a **native gate set** — the gates it runs
directly — and reported how far a circuit was from it. This lesson does the
actual work: **transpilation**, rewriting a circuit into that native basis.

Real hardware doesn't run "a Hadamard" or "a Toffoli". It runs a small fixed
set of physical operations. Everything else must be **decomposed** into that
set. The decomposition is *exact* — the transpiled circuit computes the same
thing — but it almost always uses **more gates**. That gate-count growth is one
of the central challenges of near-term quantum computing: more gates means more
noise (Lesson 21), and coherence runs out (Lesson 19).

casimirQ transpiles to the basis `{rz, ry, cx}`:

- Any single-qubit gate → a few `rz`/`ry` rotations (a general Euler
  decomposition).
- Any *singly-controlled* gate — `cx`, `cy`, `cz`, `ch`, `cp`, `crx`/`cry`/`crz`,
  or an arbitrary controlled-U — decomposes into two `cx` plus rotations (the
  standard "ABC" identity). `cx` and `cz` keep hand-optimized fast paths.
- `swap`, the Toffoli, and `cswap` (Fredkin) decompose into `cx` + rotations —
  in fact *any* multi-controlled gate does, so nothing comes back `unsupported`.

Controlled-**phase** matters especially: it's the building block of the Quantum
Fourier Transform (Lesson 11). Because the transpiler decomposes `cp`, a full
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

## Half of the story: connectivity

Decomposing to the native *gates* is only half of fitting a circuit to
hardware. The other half is **connectivity**: a real device only runs a
two-qubit gate between qubits that are physically wired together. The emulated
QPU from Lesson 22 has **linear** connectivity — a line `0—1—2—…` — so a gate
between qubits 0 and 2 simply can't run as written.

**Routing** fixes this by inserting SWAP gates to shuffle qubits until the two
operands are neighbors. `transpile_with` routes onto a connectivity and tells
you what it did:

```rust
use casq_sdk::{Connectivity, TranspileOptions};

let mut c = Circuit::new(3);
c.h(0).cx(0, 2);   // 0 and 2 aren't adjacent on a line

let t = client
    .transpile_with(&c, TranspileOptions::connectivity(Connectivity::Linear))
    .await?;
println!("inserted {} SWAP(s)", t.swap_count.unwrap());
println!("layout: {:?}", t.final_permutation.unwrap());  // logical -> physical
```

```
Routing cx(0,2) onto a linear device 0—1—2:
  inserted 1 SWAP(s)
  final layout (logical -> physical): [0, 2, 1]
  every 2-qubit gate is now between neighbors, still native: true
```

Two things to notice. First, routing **isn't free** — each SWAP is three more
`cx` gates, piling onto the gate-count problem above. Minimizing SWAPs is a
whole research area. Second, routing **permutes your qubits**: the
`final_permutation` says logical qubit 2 now lives on physical wire 1. When you
read a measurement, you look up each logical qubit's bit at
`final_permutation[logical]` — the SDK reports the layout so you always can.

### A smarter starting point

Notice we *started* with logical qubit `i` on physical qubit `i` and only then
scrambled to fix it. But we got to choose where qubits begin. If we place the
two qubits that interact — 0 and 2 — on **adjacent** wires from the start, the
gate needs no SWAP at all. That choice is the **initial layout**, and
`Layout::Greedy` picks one by seating frequently-interacting qubits close
together:

```rust
use casq_sdk::{Connectivity, Layout, TranspileOptions};

let opts = TranspileOptions::connectivity(Connectivity::Linear).with_layout(Layout::Greedy);
let t = client.transpile_with(&c, opts).await?;
println!("initial layout: {:?}", t.initial_layout.unwrap());
println!("swaps: {}", t.swap_count.unwrap());
```

```
Same circuit with a greedy initial layout:
  initial layout (logical -> physical): [1, 2, 0]
  inserted 0 SWAP(s)  <- fewer, because 0 and 2 start adjacent
```

The layout `[1, 2, 0]` puts logical qubit 0 on physical wire 1 and logical qubit
2 on physical wire 0 — neighbors — so the `cx(0, 2)` runs directly and **one
SWAP becomes zero**. Choosing a good starting layout is one of the highest-
leverage things a transpiler does; the greedy heuristic here is a first step,
not the last word (it's not guaranteed optimal), which is why it's opt-in.

### Looking ahead: the SABRE router

Even with a fixed starting layout, *how* you insert SWAPs matters. The default
router handles each gate on its own — it walks one operand to the other, which
can shove a qubit away from where the **next** gate needs it. `Router::Sabre`
(a well-known heuristic) looks ahead over a window of upcoming gates and picks
the SWAP that helps the most of them at once.

```rust
use casq_sdk::{Connectivity, Router, TranspileOptions};

// On a line, cx(0,2) then cx(0,1): greedy moves q2 (2 SWAPs); SABRE looks ahead,
// moves q0 instead, and both gates run after just 1 SWAP.
let mut c = Circuit::new(3);
c.h(0).cx(0, 2).cx(0, 1);

let greedy = client
    .transpile_with(&c, TranspileOptions::connectivity(Connectivity::Linear))
    .await?;
let sabre = client
    .transpile_with(
        &c,
        TranspileOptions::connectivity(Connectivity::Linear).with_router(Router::Sabre),
    )
    .await?;
println!("greedy: {} SWAPs, sabre: {} SWAPs",
    greedy.swap_count.unwrap(), sabre.swap_count.unwrap()); // 2 vs 1
```

Fewer SWAPs means a shallower circuit and less noise — which is the whole point
of transpiling well.

## Try it yourself

1. Transpile a GHZ preset. How does the native gate count scale with the number
   of qubits?
2. Build a larger QFT (4–5 qubits) and transpile it. Count the `cx` gates — this
   is exactly the two-qubit-gate budget that limits QFT depth on real hardware.
3. Route that QFT onto `Connectivity::Linear` and compare `swap_count` — the QFT
   entangles distant qubits, so routing on a line is expensive. Then try an
   explicit `TranspileOptions::coupling(...)` map (e.g. a ring) and see the SWAP
   count change with the topology.
4. Transpile a circuit with a `cswap` (Fredkin) gate. It comes back
   `fully_native: true` — cswap decomposes into `CX·CCX·CX`, and the Toffoli
   into rotations + `cx`. Count the gates: a single "swap if" is surprisingly
   expensive on real hardware.
5. Compare `transpiled_gate_count` for a circuit of Hadamards vs the same number
   of `cx` gates. Which "costs" more to run natively, and why?
6. Route a 4–5 qubit QFT with the default router and again with `Router::Sabre`,
   and compare `swap_count`. Where the lookahead helps depends on the circuit —
   find a case where it wins big, and one where the two tie.

## Key takeaway

Transpilation is the bridge between the circuit you *write* and the gates a
device can *run*. It's exact but not free: the gate-count blow-up it exposes is
exactly what makes deep circuits hard on real, noisy hardware — and why
shallow, hardware-aware algorithm design matters.

Back to the [overview](../README.md).
