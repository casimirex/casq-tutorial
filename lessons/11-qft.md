# Lesson 11 — Quantum Fourier Transform

**Example:** [`examples/11_qft.rs`](../examples/11_qft.rs) ·
`cargo run --example 11_qft`

## The idea

The **Quantum Fourier Transform** (QFT) is the quantum version of the discrete
Fourier transform. Where the classical FFT rearranges a list of numbers, the QFT
acts on *amplitudes* and encodes frequency information into the **phases** of a
quantum state.

Its importance is structural: the QFT is the key subroutine inside **quantum
phase estimation**, which in turn powers **Shor's algorithm** (next-next lesson)
and many others. Being able to extract periodicity efficiently is what gives
those algorithms their edge.

The circuit is elegant — for each qubit, a Hadamard followed by a "staircase" of
controlled-phase rotations from every less-significant qubit, then a final
reversal of qubit order. That's `O(n²)` gates to transform a state living in a
`2ⁿ`-dimensional space; the classical FFT over the same `2ⁿ` amplitudes takes
`O(n·2ⁿ)`.

## The code

```rust
let r = client.algorithms().qft(n).await?;
println!("gates={} depth={}", r.gate_count, r.depth);
```

We ask the server to build the QFT for several sizes and report the circuit's
gate count and depth.

## What you'll see

```
  n   gates   depth   state size
  2       ...
  6       ...
```

The gate count grows quadratically with `n` — the `n` Hadamards plus the
`n(n-1)/2` controlled-phase gates of the staircase.

## Try it yourself

1. Fit the gate counts to `a·n² + b·n`. Do the numbers match the
   Hadamard-plus-staircase count?
2. The QFT is unitary, so applying it then its inverse returns the original
   state. Sketch how you'd verify that.
3. Read up on *phase estimation* — the QFT's most important application.

## Key takeaway

The QFT turns periodicity into measurable phases with only `O(n²)` gates. It's a
subroutine, not usually an end in itself — its payoff shows up in Shor's.

Next: [Lesson 12 — Shor's algorithm](12-shor.md)
