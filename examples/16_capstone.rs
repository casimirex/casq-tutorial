//! Lesson 16 — Capstone: build your first quantum app.
//!
//! This ties the whole tutorial together into a small program that:
//!   1. builds a parameterized circuit,
//!   2. persists it on the server and lists it back,
//!   3. runs the *stored* circuit,
//!   4. draws quantum-random numbers, and
//!   5. calls a pre-built algorithm.
//!
//! It exercises the full casq-sdk surface: circuits, persistence, simulation,
//! and algorithms — the shape of a real application built on casimirQ.

use casq_sdk::{Circuit, Engine, RunOptions};
use casq_tutorial::{connect, print_histogram};

/// A "phase-kickback demonstrator": entangle a register and rotate one qubit.
fn demo_circuit(n: usize, theta: f64) -> Circuit {
    let mut c = Circuit::new(n);
    c.h(0);
    for q in 0..n - 1 {
        c.cx(q, q + 1);
    }
    c.rz(0, theta);
    c
}

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;

    // 1 + 2. Build and persist a circuit.
    let circuit = demo_circuit(3, std::f64::consts::FRAC_PI_4);
    let record = client.create_circuit("capstone-demo", &circuit).await?;
    println!("Saved circuit '{}' as {}", record.name, record.id);

    let listing = client.list_circuits(1, 5).await?;
    println!(
        "You now own {} circuit(s); newest first:",
        listing.pagination.total
    );
    for c in listing.circuits.iter().take(3) {
        println!("  - {} ({} qubits, {} ops)", c.name, c.num_qubits, c.operation_count);
    }

    // 3. Run the stored circuit by id.
    let sim = client
        .run_stored(&record.id, RunOptions::new().engine(Engine::Statevector).shots(2000))
        .await?;
    println!("\nStored-circuit run ({} shots):", sim.shots);
    print_histogram(sim.counts());

    // 4. Quantum-random 8-bit number.
    let mut coin = Circuit::new(8);
    for q in 0..8 {
        coin.h(q);
    }
    let draw = client.run(&coin, RunOptions::new().shots(1)).await?;
    let bits = draw.counts().keys().next().cloned().unwrap_or_default();
    println!(
        "\nQuantum-random byte: {} (0b{})",
        u64::from_str_radix(&bits, 2).unwrap_or(0),
        bits
    );

    // 5. Call a pre-built algorithm.
    let grover = client.algorithms().grover(4, 9, None).await?;
    println!(
        "\nGrover search (n=4, marked=9): success probability {:.4}",
        grover.success_probability
    );

    // Tidy up the circuit we created.
    client.delete_circuit(&record.id).await?;
    println!("\nCleaned up '{}'. You've built a quantum app end to end!", record.name);
    Ok(())
}
