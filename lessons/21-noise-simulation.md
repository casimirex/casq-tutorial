# Lesson 21 — Simulating noise with the density-matrix engine

**Example:** [`examples/21_noise_simulation.rs`](../examples/21_noise_simulation.rs) ·
`cargo run --example 21_noise_simulation`

## The idea

Lesson 19 introduced noise *channels* as a catalog. This lesson **runs** circuits
under them and measures the damage.

A pure state `|ψ⟩` can't represent noise — noise produces a *statistical mixture*
of states. The right object is the **density matrix** `ρ`:

- a pure state has `ρ = |ψ⟩⟨ψ|`,
- noise turns `ρ` into a blend of several `|ψ⟩⟨ψ|` terms.

The **density-matrix engine** evolves the full `ρ` (a 2ⁿ×2ⁿ matrix), so it models
noise exactly:

- gates act as `ρ → U ρ U†`,
- a noise channel acts as `ρ → Σᵢ Kᵢ ρ Kᵢ†` (its Kraus operators).

Two numbers summarize how noisy a result is:

- **Purity** `Tr(ρ²)` ∈ [1/2ⁿ, 1]. It's `1` for a pure state and drops as the
  state mixes; the maximally mixed state has purity `1/2ⁿ`.
- **Fidelity** — the overlap `⟨ψ_ideal| ρ |ψ_ideal⟩` with the noiseless result.
  `1` means "no damage"; lower means the noise pushed you off target.

## The code

```rust
use casq_sdk::advanced::{NoiseChannelConfig, NoiseSimOptions};

let mut bell = Circuit::new(2);
bell.h(0).cx(0, 1);

let r = adv.simulate_noise(
    &bell,
    &[NoiseChannelConfig::depolarizing(0.1)],  // applied after each gate
    NoiseSimOptions { compute_fidelity: true, shots: Some(2000), ..Default::default() },
).await?;
println!("purity {:.3}, fidelity {:.3}", r.purity, r.fidelity.unwrap());
```

Channel constructors: `depolarizing`, `bit_flip`, `phase_flip`,
`bit_phase_flip`, `amplitude_damping`, `phase_damping`.

## What you'll see

```
Noiseless Bell:  purity 1.000  fidelity 1.000

Depolarizing noise after every gate:
   p      purity   fidelity
  0.00    1.000    1.000
  0.10    0.680    0.817
  0.40    0.306    0.448
```

As `p` grows, both purity and fidelity fall. Under noise the Bell state even
produces the "impossible" `01` and `10` outcomes — the tell-tale sign that
correlations are decaying:

```
|11>  ~44%   |00>  ~42%   |01>  ~7%   |10>  ~7%
```

And amplitude damping relaxes `|1>` toward `|0>` by exactly `gamma`:

```
  gamma 0.50 -> P(0) = 0.500
  gamma 1.00 -> P(0) = 1.000
```

## Try it yourself

1. Swap depolarizing for `phase_damping`. It kills coherence (off-diagonal terms)
   but not the `00`/`11` populations — how do purity and the counts respond
   differently from depolarizing?
2. Stack two channels: `[depolarizing(0.05), amplitude_damping(0.05)]`. Noise
   compounds.
3. Because the engine holds a 4ⁿ matrix, it caps at ~10 qubits. Why is that the
   fundamental cost of exact noise simulation — and why do the variational
   algorithms (Lessons 13–16) avoid needing it?

## Key takeaway

The density-matrix engine is your microscope for noise: it quantifies exactly
how much a real device would degrade a result. Purity and fidelity turn "is this
circuit robust?" into numbers you can track — the foundation of noise-aware
quantum programming.

Back to the [overview](../README.md).
