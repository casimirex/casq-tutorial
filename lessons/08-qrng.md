# Lesson 8 — A Quantum Random Number Generator

**Example:** [`examples/08_qrng.rs`](../examples/08_qrng.rs) ·
`cargo run --example 08_qrng`

## The idea

Most "random" numbers in software are **pseudo-random**: a deterministic
algorithm expands a seed into a stream that only *looks* random. Given the seed,
the whole sequence is predictable. For simulations that's fine; for
cryptography, predictability is a liability.

Quantum measurement offers randomness grounded in physics. Put `n` qubits into
superposition, measure once, and read the `n`-bit result as an integer. Each bit
is an independent fair coin, so you get a uniform random value in `[0, 2ⁿ)` — and
the outcome is fundamentally unpredictable, not merely hard to predict.

## The code

```rust
async fn quantum_random(client: &Client, n: usize) -> Result<u64> {
    let mut circuit = Circuit::new(n);
    for q in 0..n { circuit.h(q); }         // uniform superposition
    let result = client.run(&circuit, RunOptions::new().shots(1)).await?;
    let bits = result.counts().keys().next().cloned().unwrap();
    Ok(u64::from_str_radix(&bits, 2).unwrap()) // bitstring -> integer
}
```

A single shot yields exactly one bitstring, which we parse as binary.

We also build a **quantum die** using *rejection sampling*: draw 3 bits (values
0–7) and discard `0` and `7`, keeping a uniform `1–6`. Rejection sampling is the
standard trick for turning a power-of-two range into any range without bias.

## What you'll see

```
Ten quantum-random bytes (0..255):
   92  148   1  92  80  69  95  16  107  72

Rolling a fair quantum die five times:
  3 5 3 2 1
```

## Try it yourself

1. Generate 1000 single-bit draws and count 0s vs 1s — it should be ~50/50.
2. Extend the die to a 20-sided die. How many bits do you need, and what's the
   rejection rate?
3. One `shots(k)` run returns `k` samples at once. Rewrite `quantum_random` to
   draw a whole batch in a single call for efficiency.

## Key takeaway

This is the smallest genuinely useful quantum program. It also cements the core
loop: prepare a state, sample it, interpret the classical bits.

Next: [Lesson 9 — Deutsch–Jozsa](09-deutsch-jozsa.md)
