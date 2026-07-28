# Lesson 25 — The Circuit Library: building blocks & controlled rotations

**Example:** [`examples/25_circuit_library.rs`](../examples/25_circuit_library.rs) ·
`cargo run --example 25_circuit_library`

## The idea

Every algorithm in this course — Deutsch–Jozsa, Grover, Shor, VQE — is a clever
arrangement of a small set of **building-block circuits**. CasimirQ ships 53 of
them as a browsable **Circuit Library** (its own section in the web app). Each
is a real saved circuit, verified on the engine, that you can open straight into
the builder — or rebuild here with the same fluent `casq_sdk` API.

Lessons 3 and 6 toured the single- and two-qubit gates. This lesson fills in the
pieces we hadn't driven yet, so the SDK can express the *whole* Library:

- the **controlled rotations** `crx` / `cry` / `crz`,
- **controlled-Hadamard** (`ch`), and
- a reusable **subroutine** — the swap test.

> **Bit order.** Measurement strings print with **qubit 0 on the right**, so
> `|11>` means `q0=1, q1=1`.

## Controlled rotations (`crx`, `cry`, `crz`)

A controlled rotation applies `Rx/Ry/Rz(θ)` to the target **only when the
control is `|1>`** — the continuous-angle cousins of CNOT/CZ. They are the
workhorses of variational circuits (VQE, QAOA) and of phase estimation.

`Ry(π)` maps `|0> → |1>`, so a *controlled* `Ry(π)` flips the target exactly when
the control is set:

```rust
let mut c = Circuit::new(2);
c.x(0).cry(0, 1, std::f64::consts::PI);   // q0=1 -> Ry(π) on q1
// |01> -> |11>
```

`Rz` only adds a **phase**, which you can't read directly — so we make it visible
with interference (put the target in `|+>`, apply the controlled `Rz(π)`, then
`H`; the hidden phase becomes a bit you can measure):

```rust
let mut c = Circuit::new(2);
c.x(0).h(1).crz(0, 1, std::f64::consts::PI).h(1);
// control set -> q1 reads 1
```

## Controlled-Hadamard (`ch`)

Put the target into superposition **on demand** — Hadamard the target when the
control is `|1>`:

```rust
let mut c = Circuit::new(2);
c.x(0).ch(0, 1);          // q1 becomes 50/50 -> |01> and |11>
```

## A subroutine: the swap test

Building blocks compose into *subroutines* that show up inside bigger algorithms.
The **swap test** measures whether two states are equal. An ancilla controls a
swap of the two registers, sandwiched by Hadamards; if the states are identical
the interference is perfect and the ancilla always reads `0`:

```rust
let mut c = Circuit::new(3);
c.h(0).cswap(0, 1, 2).h(0);   // q1 = q2 = |0> -> ancilla q0 = 0
```

## What you'll see

```
CRY(pi)  |01> -> |11>   (control q0=1 -> Ry(pi) flips q1):
  |11>    1000  100.0%  ########################################

CRY(pi)  |00> -> |00>   (control q0=0 -> no rotation):
  |00>    1000  100.0%  ########################################

CRZ(pi)  kickback with control q0=1 -> q1 reads 1  (|11>):
  |11>    1000  100.0%  ########################################

CH       control q0=1 -> H on q1 -> 50/50 over |01>,|11>:
  |11>     506   50.6%  ########################################
  |01>     494   49.4%  #######################################

SWAP TEST  identical inputs -> |000|, ancilla q0 = 0 (states match):
  |000>    1000  100.0%  ########################################
```

## Try it yourself

1. Sweep the angle: run `cry(0, 1, θ)` for `θ = 0, π/2, π` and watch the target's
   `|1>` probability follow `sin²(θ/2)`.
2. Feed the swap test *different* states (`x(2)` so `q1≠q2`) and confirm the
   ancilla now reads `1` some of the time — the overlap dropped.
3. Rebuild a Library subroutine end to end: a **phase oracle** (`h;h;cz` marks
   `|11>`) plus **Grover diffusion** (`h;x;cz;x;h`) is one amplitude-amplification
   step (Lesson 10). Compare against opening the same circuit from the web app's
   Circuit Library.
4. Transpile a controlled rotation (Lesson 24) and count the native two-qubit
   gates — continuous controlled gates aren't free on hardware.

## Key takeaway

The Circuit Library is the periodic table of quantum computing: **53 small,
verified circuits** — gates, state preparations, measurements, and subroutines —
that everything larger is built from. With `crx/cry/crz` added, the `casq_sdk`
fluent API can express every one of them, and the same circuit you draw in the
web app you can script here.
