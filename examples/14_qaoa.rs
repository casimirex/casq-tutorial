//! Lesson 14 — Quantum Approximate Optimization Algorithm (QAOA).
//!
//! QAOA attacks combinatorial optimization (here MaxCut: split a graph's
//! vertices into two groups so that as many edges as possible cross the cut).
//! Like VQE it is variational — alternating "cost" and "mixer" layers with
//! tunable angles that a classical optimizer improves. More layers (larger p)
//! can yield better approximations at the cost of a deeper circuit.
//!
//! We run QAOA on the server's example graphs.

use casq_tutorial::connect;

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let algos = client.algorithms();

    let graphs = algos.qaoa_examples().await?;
    let mut names: Vec<&String> = graphs.keys().collect();
    names.sort();

    println!("MaxCut via QAOA on example graphs:\n");
    for name in names {
        let g = &graphs[name];
        let r = algos.qaoa(g.n, &g.edges, Some(1)).await?;
        println!(
            "  {name:<8} ({} vertices, {} edges): best cut = {}  (expectation {:.3})",
            g.n,
            g.edges.len(),
            r.best_cut_value,
            r.max_expectation
        );
    }

    println!("\nMaxCut is NP-hard; QAOA aims for good *approximate* cuts, and is a");
    println!("popular benchmark for near-term quantum optimization.");
    Ok(())
}
