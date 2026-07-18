//! Lesson 6 — Simulation engines and scaling.
//!
//! casimirQ ships several engines. The dense *statevector* engine is exact and
//! general but its memory doubles with every qubit. The *Clifford* engine is
//! restricted to a gate set (H, S, CNOT, Pauli) yet scales to many more qubits
//! because it tracks stabilizers instead of amplitudes. `Engine::Auto` lets the
//! server pick. This lesson runs the same Clifford-friendly circuit under
//! different engines and shows they agree.

use casq_sdk::{Circuit, Engine, RunOptions};
use casq_tutorial::{connect, print_histogram};

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;

    // A GHZ state on 5 qubits: only H and CNOT, so it is a Clifford circuit.
    let n = 5;
    let mut circuit = Circuit::new(n);
    circuit.h(0);
    for q in 0..n - 1 {
        circuit.cx(q, q + 1);
    }

    for engine in [Engine::Auto, Engine::Statevector, Engine::Clifford] {
        let result = client
            .run(&circuit, RunOptions::new().engine(engine).shots(2000))
            .await?;
        println!(
            "engine {:?} -> ran as {:?}, {:.4} ms",
            engine, result.requested_engine, result.metadata.execution_time_ms
        );
        print_histogram(result.counts());
        println!();
    }

    println!("All three engines produce the same GHZ distribution.");
    println!("Rule of thumb: reach for Clifford when your circuit uses only");
    println!("H, S, CNOT and Pauli gates and you need many qubits.");
    Ok(())
}
