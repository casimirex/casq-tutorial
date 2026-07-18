//! Lesson 5 — Multi-qubit entanglement: the GHZ state.
//!
//! Entanglement is not limited to two qubits. Chaining CNOTs out from a qubit in
//! superposition builds the n-qubit GHZ state (|00..0> + |11..1>)/sqrt(2): all
//! qubits collapse to the same value together.

use casq_sdk::{Circuit, Engine, RunOptions};
use casq_tutorial::{connect, print_histogram};

/// Build an n-qubit GHZ state: H on qubit 0, then a CNOT ladder.
fn ghz(n: usize) -> Circuit {
    let mut c = Circuit::new(n);
    c.h(0);
    for q in 0..n - 1 {
        c.cx(q, q + 1);
    }
    c
}

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;

    for n in [3, 4, 5] {
        let circuit = ghz(n);
        let result = client
            .run(&circuit, RunOptions::new().engine(Engine::Statevector).shots(2000))
            .await?;

        println!("GHZ state on {n} qubits:");
        print_histogram(result.counts());

        let all_zero = "0".repeat(n);
        let all_one = "1".repeat(n);
        let clean = result
            .counts()
            .keys()
            .all(|s| *s == all_zero || *s == all_one);
        println!("Only all-0 and all-1 states appear: {clean}\n");
    }
    Ok(())
}
