//! Lesson 13 — Variational Quantum Eigensolver (VQE).
//!
//! VQE is a hybrid quantum-classical algorithm: a quantum device prepares a
//! parameterized trial state and measures its energy against a Hamiltonian,
//! while a classical optimizer tweaks the parameters to drive that energy down
//! toward the ground state. It is a leading candidate for near-term quantum
//! chemistry and materials science.
//!
//! We seed VQE with a built-in example Hamiltonian and let the server run the
//! optimization loop.

use casq_tutorial::connect;

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let algos = client.algorithms();

    // The server provides ready-made example Hamiltonians (name -> Pauli terms).
    let examples = algos.vqe_examples().await?;
    let mut names: Vec<&String> = examples.keys().collect();
    names.sort();

    println!("Estimating ground-state energies with VQE:\n");
    for name in names {
        let hamiltonian = &examples[name];
        // Qubit count = highest qubit index referenced by any term, plus one.
        let n = hamiltonian
            .iter()
            .flat_map(|t| t.qubits.iter().copied())
            .max()
            .map_or(1, |m| m + 1);

        let r = algos.vqe(n, hamiltonian, Some(150)).await?;
        println!(
            "  {name:<10} ({n} qubits): ground-state energy ~= {:+.4}  (converged: {})",
            r.optimal_energy, r.converged
        );
    }

    println!("\nVQE never diagonalizes the full Hamiltonian; it *searches* for the");
    println!("lowest energy, which is why it fits noisy, near-term hardware.");
    Ok(())
}
