# Lesson 18 — Quantum error correction

**Example:** [`examples/18_error_correction.rs`](../examples/18_error_correction.rs) ·
`cargo run --example 18_error_correction`

## The idea

Qubits are delicate. They lose their state to the environment (decoherence) and
every gate is slightly imperfect. **Quantum error correction (QEC)** is how a
quantum computer will eventually run long algorithms despite this — and it's the
bridge between today's noisy machines and the fault-tolerant future that Shor's
algorithm needs.

You can't just copy a qubit to back it up (no-cloning), and you can't peek at it
to check for errors (measurement destroys it). QEC sidesteps both:

- **Encode** one *logical* qubit across many *physical* qubits.
- Measure **stabilizers** — cleverly chosen joint checks that reveal *whether* an
  error happened, and where, **without** revealing the encoded data.

The measured check pattern is the **syndrome**. An all-zero syndrome means "no
error"; a specific non-zero pattern tells the decoder exactly which correction to
apply. A code's **distance** `d` sets its power: it can correct up to
`(d-1)/2` errors.

Two classic codes:

| Code | Physical qubits | Logical | Distance |
| --- | --- | --- | --- |
| Steane | 7 | 1 | 3 |
| Shor | 9 | 1 | 3 |

Both are distance 3, so each corrects any single-qubit error.

## The code

```rust
let adv = client.advanced();
for code in adv.qec_codes().await? {
    // code.n_physical, code.n_logical, code.distance, ...
}
let encoded = adv.encode("steane", Some(&[0])).await?;   // encode logical |0>
let syn = adv.syndrome("steane", Some(&[0])).await?;      // measure the check
```

## What you'll see

```
  steane  7 physical -> 1 logical, distance 3 (corrects up to 1 error(s))
  shor    9 physical -> 1 logical, distance 3 (corrects up to 1 error(s))

  measured syndrome [0, 0, 0, 0, 0, 0] -> all zero: no error detected
```

The freshly encoded state is clean, so all six Steane stabilizers read zero.

## Try it yourself

1. Compare the codes' overhead: Steane spends 7 qubits, Shor 9, both to protect
   one. Why is that ratio the central challenge of building a real machine?
2. Look up how many physical qubits estimates suggest are needed per *logical*
   qubit for useful algorithms (hint: it's large).
3. Relate `distance` to `(d-1)/2`: what distance would you need to correct two
   simultaneous errors?

## Key takeaway

QEC trades many noisy physical qubits for one reliable logical qubit, using
stabilizer measurements that catch errors without reading the data. It's the
linchpin of scalable quantum computing.

Next: [Lesson 19 — Noise & the NISQ reality](19-noise.md)
