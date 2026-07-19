# Lesson 17 — Quantum machine learning

**Example:** [`examples/17_quantum_ml.rs`](../examples/17_quantum_ml.rs) ·
`cargo run --example 17_quantum_ml`

## The idea

Quantum machine learning asks: can quantum states represent data in ways that
make patterns easier to separate? Two near-term approaches lead the field, and
`casq-sdk`'s `advanced()` handle exposes both.

### Quantum kernels (QSVM)

A **feature map** is a circuit that embeds a classical data point `x` into a
quantum state `|φ(x)>`. The **kernel** between two points is the overlap
`|⟨φ(x)|φ(x')⟩|²` — a similarity score. Compute that for every pair and you get a
**kernel matrix**; hand it to an ordinary classical Support Vector Machine and
you have a **Quantum Support Vector Machine (QSVM)**. The hope is that a quantum
feature map reaches similarity structure a classical kernel can't cheaply match.

### Variational models (VQE-style)

The same variational loop from Lesson 14 — a parameterized **ansatz** circuit
plus a classical optimizer — is the training engine for many quantum models.
Here we pick an ansatz and minimize an energy.

## The code

```rust
let adv = client.advanced();

// Quantum kernel matrix over a 4-point, two-cluster dataset.
let kernel = adv.kernel_matrix(&data, Some("zz")).await?;

// Variational optimization with a chosen ansatz.
use casq_sdk::advanced::{MlPauliTerm, VqeRunOptions};
let vqe = adv.ml_vqe(
    &[MlPauliTerm::new("ZZ", 1.0), MlPauliTerm::new("XX", 0.5)],
    "hardware_efficient",
    VqeRunOptions { max_iterations: Some(60), ..Default::default() },
).await?;
```

Note the ML Hamiltonian uses a single **Pauli string** per term (`"ZZ"`), which
differs from the per-qubit Pauli terms in Lesson 14's algorithms API.

## What you'll see

```
Quantum kernel matrix (feature map: zz):
  [1.000, 1.000, 0.868, 0.861]
  [1.000, 1.000, 0.862, 0.854]
  [0.868, 0.862, 1.000, 0.999]
  [0.861, 0.854, 0.999, 1.000]
```

The two upper-left and two lower-right points (the clusters) score ~1.0 with each
other and lower across the divide — the kernel *sees* the two classes. The
diagonal is exactly 1.0: every point is identical to itself.

## Try it yourself

1. Switch the feature map to `"pauli"`. Does the separation change?
2. Move the clusters closer together. At what point does the kernel stop
   distinguishing them?
3. Compare the two ansatze from `ml_catalog()` — how do their parameter counts
   differ, and why might more parameters help or hurt?

## Key takeaway

Quantum ML reuses everything you've learned — feature maps are circuits, kernels
are overlaps, training is variational optimization. It's one of the most active
areas of near-term quantum research.

Next: [Lesson 18 — Quantum error correction](18-error-correction.md)
