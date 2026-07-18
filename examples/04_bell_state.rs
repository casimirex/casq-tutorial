//! Lesson 4 — Entanglement: the Bell state.
//!
//! A Hadamard on qubit 0 followed by a CNOT (control 0, target 1) creates the
//! Bell pair (|00> + |11>)/sqrt(2). The two qubits are now entangled: measuring
//! one instantly determines the other. You will only ever see "00" or "11" —
//! never "01" or "10".

use casq_sdk::{Circuit, Engine, RunOptions};
use casq_tutorial::{connect, print_histogram};

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;

    let mut bell = Circuit::new(2);
    bell.h(0).cx(0, 1);

    let result = client
        .run(&bell, RunOptions::new().engine(Engine::Statevector).shots(2000))
        .await?;

    println!("Bell state (|00> + |11>)/sqrt(2):");
    print_histogram(result.counts());

    let correlated = result
        .counts()
        .keys()
        .all(|s| s == "00" || s == "11");
    println!(
        "\nPerfectly correlated (only 00 and 11 appear): {}",
        correlated
    );
    Ok(())
}
