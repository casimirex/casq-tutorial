//! Lesson 17 — Quantum machine learning: kernels and VQE.
//!
//! Two ideas power much of near-term quantum ML:
//!
//! 1. **Quantum kernels.** A feature map embeds classical data into a quantum
//!    state; the kernel between two points is the overlap of their states. Feed
//!    that kernel to a classical SVM and you have a Quantum Support Vector
//!    Machine (QSVM). We compute a kernel (Gram) matrix and see that points in
//!    the same cluster are far more similar than points in different clusters.
//!
//! 2. **Variational VQE for ML.** A parameterized "ansatz" circuit plus a
//!    classical optimizer minimizes an energy — the same machinery that trains
//!    variational models.

use casq_tutorial::connect;

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let adv = client.advanced();

    // What's available?
    let catalog = adv.ml_catalog().await?;
    println!("Ansatze:");
    for a in &catalog.ansatze {
        println!("  - {} ({} qubits, {} params, {} entanglement)", a.id, a.n_qubits, a.n_params, a.entanglement);
    }
    println!("Feature maps: {:?}\n", catalog.feature_maps);

    // A tiny two-cluster dataset: rows 0-1 near the origin, rows 2-3 far away.
    let data = vec![
        vec![0.10, 0.15], // class A
        vec![0.12, 0.09], // class A
        vec![0.90, 0.85], // class B
        vec![0.88, 0.92], // class B
    ];
    let kernel = adv.kernel_matrix(&data, Some("zz")).await?;

    println!("Quantum kernel matrix (feature map: {}):", kernel.feature_map);
    for row in &kernel.matrix {
        let cells: Vec<String> = row.iter().map(|v| format!("{v:.3}")).collect();
        println!("  [{}]", cells.join(", "));
    }
    let intra = kernel.matrix[0][1];
    let inter = kernel.matrix[0][2];
    println!(
        "\nSame-cluster similarity {:.3} vs different-cluster {:.3} -> the kernel",
        intra, inter
    );
    println!("separates the classes, which is exactly what a QSVM exploits.");

    // Variational VQE over a small Hamiltonian.
    use casq_sdk::advanced::{MlPauliTerm, VqeRunOptions};
    let hamiltonian = vec![MlPauliTerm::new("ZZ", 1.0), MlPauliTerm::new("XX", 0.5)];
    let vqe = adv
        .ml_vqe(&hamiltonian, "hardware_efficient", VqeRunOptions { max_iterations: Some(60), ..Default::default() })
        .await?;
    println!(
        "\nVQE ({}): min energy {:.4} over {} params in {} iterations (converged: {})",
        vqe.ansatz, vqe.min_energy, vqe.optimal_params.len(), vqe.iterations, vqe.converged
    );
    Ok(())
}
