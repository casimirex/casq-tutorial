# Lesson 11 — Shor's algorithm

**Example:** [`examples/11_shor.rs`](../examples/11_shor.rs) ·
`cargo run --example 11_shor`

## The idea

**Shor's algorithm** factors large integers in *polynomial* time — exponentially
faster than the best known classical methods. That matters far beyond math: RSA,
the public-key cryptography securing much of the internet, rests on factoring
being hard. A large, fault-tolerant quantum computer running Shor's would break
it. This single result is why "post-quantum cryptography" is now a serious field.

The clever part is that Shor's is mostly *classical*. It reduces factoring `N` to
**period finding**: pick a random `a` coprime to `N`, and find the period `r` of
`f(x) = aˣ mod N`. Once you know `r` (and it's even), `gcd(a^(r/2) ± 1, N)` very
likely yields a real factor — plain number theory. The *only* quantum step is
finding `r`, which quantum phase estimation (built on the QFT from the last
lesson) does efficiently.

## The code

```rust
let r = client.algorithms().shor(n).await?;
let product: i64 = r.factors.iter().product();  // should equal n
```

We factor a few semiprimes and verify the factors multiply back.

## What you'll see

```
N = 15  ->  factors [5, 3]  (product 15, attempts 1, nontrivial: true)
N = 21  ->  factors [7, 3]  (product 21, attempts 1, nontrivial: true)
N = 35  ->  factors [5, 7]  (product 35, attempts 1, nontrivial: true)
```

## Try it yourself

1. Verify each result: are the factors prime, and do they multiply to `N`?
2. Why must `N` be odd and not a prime power for this to be interesting? (Those
   cases have easy classical shortcuts.)
3. Read one sentence of RSA's definition and note exactly which step Shor's
   attacks.

## A note on scale

Factoring `15` needs only a few qubits; factoring a real 2048-bit RSA modulus
needs millions of high-quality qubits and error correction we don't yet have.
Shor's is a proof of *possibility* driving today's cryptographic migration, not
a present-day threat.

## Key takeaway

A mostly-classical algorithm with one quantum subroutine (period finding via the
QFT) turns a "hard" problem easy — and reshaped modern cryptography.

Next: [Lesson 12 — Quantum teleportation](12-teleportation.md)
