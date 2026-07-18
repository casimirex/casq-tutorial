# Lesson 18 — Noise & the NISQ reality

**Example:** [`examples/18_noise.rs`](../examples/18_noise.rs) ·
`cargo run --example 18_noise`

## The idea

We live in the **NISQ** era — Noisy Intermediate-Scale Quantum. Real devices have
tens to hundreds of qubits with no full error correction yet, so **noise** is the
dominant limit on what you can compute. Being noise-aware is a practical skill,
not an afterthought.

Errors are modeled as **channels** that act on qubits:

| Channel | What it does | Physical cause |
| --- | --- | --- |
| depolarizing | applies a random Pauli error with some probability | generic gate error |
| amplitude damping | `|1>` decays toward `|0>` | energy loss (T1) |
| phase damping | destroys phase coherence | dephasing (T2) |
| bit flip / phase flip | flips value or sign | specific error types |

A device is summarized by **characteristics**: qubit count, connectivity, gate
times, error rates, and the coherence times **T1** (how long a qubit holds
energy) and **T2** (how long it holds phase). Those times cap how deep a circuit
can be before noise washes out the signal.

## The code

```rust
let adv = client.advanced();
let catalog = adv.noise_catalog().await?;        // channels + device models

use casq_sdk::advanced::NoiseChannel;
let channels = vec![
    NoiseChannel::new("depolarizing", ("probability", 0.01), 0),
    NoiseChannel::new("amplitude_damping", ("gamma", 0.02), 1),
];
let validation = adv.validate_noise(&channels).await?;

let dev = adv.characterize("ibmq_lagos").await?;  // T1/T2, error rates, ...
```

## What you'll see

```
Supported noise channels: ["depolarizing", "amplitude_damping", "phase_damping", "bit_flip", "phase_flip"]
Built-in device models:   ["ideal", "depolarizing", "ibmq_lagos"]
Noise spec valid: true
```

plus the qubit count reported for each device model.

## Try it yourself

1. Add a channel with an out-of-range parameter (e.g. probability `2.0`) and
   confirm `validate_noise` flags it.
2. Compare the `ideal` model with a noisy one — what's different?
3. Connect this back to Lesson 13/16: *why* are shallow variational circuits the
   right shape for noisy hardware?

## Key takeaway

Noise defines the NISQ era. Knowing the channels and a device's coherence limits
tells you which circuits are realistic — and pushes you toward shallow,
error-tolerant designs until full error correction (Lesson 17) matures.

Next: [Lesson 19 — BB84 quantum key distribution](19-bb84.md)
