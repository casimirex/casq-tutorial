# Lesson 14 — Variational Quantum Eigensolver (VQE)

**Example:** [`examples/14_vqe.rs`](../examples/14_vqe.rs) ·
`cargo run --example 14_vqe`

## The idea

The algorithms so far assume clean, deep circuits. Today's real hardware is
noisy and shallow. **Variational** algorithms are designed for exactly that
regime, and VQE is the flagship.

VQE estimates the **ground-state energy** of a system — the lowest eigenvalue of
its Hamiltonian `H`. That number is central to chemistry and materials science:
it tells you molecular binding energies, reaction rates, and material properties.

It works as a **hybrid loop**:

1. A quantum device prepares a trial state `|ψ(θ)>` from tunable parameters `θ`
   (a shallow "ansatz" circuit).
2. It measures the energy `⟨ψ(θ)| H |ψ(θ)⟩`.
3. A *classical* optimizer adjusts `θ` to lower that energy.
4. Repeat until it converges.

The **variational principle** guarantees the measured energy is never below the
true ground state, so minimizing it drives you toward the answer. The quantum
computer only does the hard part (preparing and measuring the state); the
optimization is ordinary classical code.

A Hamiltonian is supplied as a weighted sum of **Pauli terms** — each a
coefficient plus which Pauli operators act on which qubits.

## The code

```rust
let examples = client.algorithms().vqe_examples().await?; // name -> Pauli terms
let hamiltonian = &examples["H2"];
let n = hamiltonian.iter().flat_map(|t| t.qubits.iter().copied())
    .max().map_or(1, |m| m + 1);
let r = client.algorithms().vqe(n, hamiltonian, Some(150)).await?;
println!("ground-state energy ~= {:.4}", r.optimal_energy);
```

## What you'll see

```
  H2       (2 qubits): ground-state energy ~= -1.13  (converged: true)
  simple_1 (1 qubits): ground-state energy ~= -1.00  (converged: true)
```

Because VQE *searches*, the exact number varies a little between runs and depends
on the optimizer and starting point — that's the nature of a variational method,
not a bug.

## Try it yourself

1. Run it several times. How much does the H₂ energy move between runs?
2. Raise `max_iterations`. Does convergence improve?
3. Inspect a Hamiltonian's Pauli terms and identify which qubits each acts on.

## Key takeaway

Variational algorithms trade deep, exact circuits for shallow, tunable ones plus
classical optimization — the pragmatic path for near-term ("NISQ") hardware.

Next: [Lesson 15 — QAOA](15-qaoa.md)
