# Lesson 24 — Multi-qubit gates: CNOT, CZ, SWAP, Toffoli, Fredkin

**Example:** [`examples/24_multi_qubit_gates.rs`](../examples/24_multi_qubit_gates.rs) ·
`cargo run --example 24_multi_qubit_gates`

## The idea

Single-qubit gates (Lesson 3) turn one qubit on the Bloch sphere. But a quantum
computer's power comes from gates that **couple** qubits — where one qubit's
value decides what happens to another. Those controlled interactions are how
entanglement is created (Lessons 4–5) and how quantum logic is built.

This lesson is a hands-on tour of the core multi-qubit gates. For each one we
feed in a definite computational-basis state and read the definite output, so
the gate's rule is unmistakable.

> **Bit order.** Measurement strings print with **qubit 0 on the right**, so
> `|011>` means `q0=1, q1=1, q2=0`. Every arrow below is written the same way.

## The gate zoo

### CNOT (controlled-X, `cx`)
The workhorse. Flip the **target** if and only if the **control** is `|1>`:

```rust
let mut c = Circuit::new(2);
c.x(0).cx(0, 1);          // q0=1, then CNOT(control=q0, target=q1)
// |01> -> |11>
```

`q2 = q2 XOR control` — a reversible XOR. On a control in superposition it
produces entanglement: that's exactly the Bell state of Lesson 4.

### CZ (controlled-Z, `cz`)
Applies a **phase** of −1 to `|11>` and nothing else. It is *symmetric* — swap
control and target and it's the same gate — and diagonal, so on a basis state
the phase is invisible to measurement. To *see* it, turn the phase into
interference with Hadamards (phase kickback):

```rust
let mut c = Circuit::new(2);
c.h(0).x(1).cz(0, 1).h(0);   // q0=|+>, q1=|1>; CZ kicks a Z back onto q0
// q0 flips to 1 — the "hidden" phase became a measurable bit
```

### SWAP (`swap`)
Exchanges two qubits. Useful for moving data to where a two-qubit gate can act
(the routing problem of Lesson 23). It's three CNOTs in a trench coat.

```rust
let mut c = Circuit::new(2);
c.x(0).swap(0, 1);        // |01> -> |10>
```

### Toffoli (CCNOT, `ccx`)
Two controls, one target: flip the target iff **both** controls are `|1>`. That
is a reversible **AND** (`q2 = q2 XOR (q0 AND q1)`), which makes Toffoli enough
to build any classical logic circuit reversibly.

```rust
let mut c = Circuit::new(3);
c.x(0).x(1).ccx(0, 1, 2); // |011> -> |111>
```

### Fredkin (CSWAP, `cswap`)
One control, two targets: swap the targets iff the control is `|1>`. A
controlled-SWAP — also universal for reversible classical computation.

```rust
let mut c = Circuit::new(3);
c.x(0).x(1).cswap(0, 1, 2); // control q0=1 -> swap q1,q2:  |011> -> |101>
```

## What you'll see

```
CNOT  |01> -> |11>   (control q0=1 flips target q1):
  |11>    1000  100.0%  ########################################

CZ    kickback: H·(CZ, target q1=1)·H -> q0 reads 1  (|11>):
  |11>    1000  100.0%  ########################################

SWAP  |01> -> |10>   (q0 and q1 trade values):
  |10>    1000  100.0%  ########################################

CCX   |011> -> |111>  (both controls 1 -> flip target q2):
  |111>   1000  100.0%  ########################################

CSWAP |011> -> |101>  (control q0=1 -> swap q1 and q2):
  |101>   1000  100.0%  ########################################
```

Each gate lands the input on exactly the output its rule predicts.

## Try it yourself

1. Put the CNOT's control in superposition (`h(0)` instead of `x(0)`) and
   measure. You've rebuilt the Bell state — CNOT + superposition = entanglement.
2. Show CZ is symmetric: `cz(0, 1)` and `cz(1, 0)` give identical results for
   every input. Why does that make sense from its `|11> -> -|11>` rule?
3. Build a half-adder: two data qubits and a carry. Use a Toffoli for the carry
   bit and a CNOT for the sum. Verify all four input combinations.
4. Transpile a Toffoli and a Fredkin (Lesson 23) and compare gate counts — a
   "single" three-qubit gate is far from free on real hardware.

## Key takeaway

Multi-qubit gates are where the interesting physics lives. **CNOT** and **CZ**
are the two-qubit workhorses (and turn superposition into entanglement); **SWAP**
moves data; **Toffoli** and **Fredkin** are reversible AND / controlled-swap,
enough to express *any* classical logic reversibly. Everything larger — the
algorithms of Part III — is built from exactly these couplings.

Back to the [overview](../README.md).
