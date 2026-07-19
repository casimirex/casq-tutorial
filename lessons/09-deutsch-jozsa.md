# Lesson 9 — Deutsch–Jozsa

**Example:** [`examples/09_deutsch_jozsa.rs`](../examples/09_deutsch_jozsa.rs) ·
`cargo run --example 09_deutsch_jozsa`

## The idea

This is the first algorithm where quantum beats classical *provably*, and it
introduces a pattern you'll see everywhere: the **oracle**.

You're handed a black-box function `f` on `n` input bits, promised to be one of:

- **constant** — same output (all 0 or all 1) for every input, or
- **balanced** — 0 for exactly half the inputs and 1 for the other half.

Classically, to be *certain* which it is you may need to test just over half of
the `2ⁿ` inputs. Deutsch–Jozsa answers with **one** evaluation of `f`.

The trick is **phase kickback**. Prepare an ancilla qubit in `|->` (via `X` then
`H`). When the oracle computes `y → y ⊕ f(x)` into that ancilla, the effect is to
stamp a `(-1)^f(x)` *phase* onto each input branch instead of changing the
ancilla. A final layer of Hadamards turns those phases into interference: if `f`
is constant, everything reinforces at the all-zeros string; if `f` is balanced,
the all-zeros amplitude cancels to exactly zero.

**Read-out rule:** measure the input register. All zeros ⇒ constant. Anything
else ⇒ balanced.

## The code

We build three oracles ourselves:

```rust
match oracle {
    Oracle::ConstantZero => { /* identity */ }
    Oracle::ConstantOne  => { c.x(ancilla); }          // flip y for all x
    Oracle::Balanced     => { for q in 0..n { c.cx(q, ancilla); } } // parity
}
```

`CNOT` from every input into the ancilla computes the parity `x0 ⊕ x1 ⊕ …`,
which is balanced.

## What you'll see

```
oracle: constant (always 0)  -> input register 0000 => CONSTANT
oracle: constant (always 1)  -> input register 1000 => CONSTANT
oracle: balanced (parity)    -> input register 1111 => BALANCED
```

(The leftmost bit is the ancilla; we only inspect the input qubits, so the
"always 1" case is still all-zeros on the inputs → CONSTANT.)

## Try it yourself

1. Add a balanced oracle that XORs only *some* inputs into the ancilla. Still
   balanced?
2. Increase `n` to 5. The classical worst case jumps to 17 queries; the quantum
   cost stays at one.
3. Print the full statevector before the final Hadamards to *see* the phases.

## Key takeaway

Oracles + phase kickback + interference is the template. Grover and Shor are
more sophisticated members of the same family.

Next: [Lesson 10 — Grover's search](10-grover.md)
