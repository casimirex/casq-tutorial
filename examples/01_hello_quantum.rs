//! Lesson 1 — Hello, Quantum: your first superposition.
//!
//! A single qubit starts in |0>. A Hadamard gate puts it into an equal
//! superposition of |0> and |1>, so measuring it yields 0 or 1 with ~50%
//! probability each. Run it and watch a fair quantum coin.

use casq_sdk::{Circuit, Engine, RunOptions};
use casq_tutorial::{connect, print_histogram};

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;

    // One qubit, one Hadamard.
    let mut circuit = Circuit::new(1);
    circuit.h(0);

    let result = client
        .run(&circuit, RunOptions::new().engine(Engine::Statevector).shots(1000))
        .await?;

    println!("A qubit in superposition, measured {} times:", result.shots);
    print_histogram(result.counts());

    println!("\nExact amplitudes from the statevector:");
    for amp in result.statevector() {
        println!(
            "  |{}>: amplitude {:+.4}{:+.4}i  (probability {:.3})",
            amp.state, amp.re, amp.im, amp.probability
        );
    }
    Ok(())
}
