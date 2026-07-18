//! Lesson 7 — A Quantum Random Number Generator.
//!
//! Classical "random" numbers are usually pseudo-random: deterministic streams
//! seeded once. Measuring qubits in superposition gives randomness rooted in
//! physics. Here we put n qubits into superposition, take a single shot, and
//! read the bitstring as an integer — a genuine random number in [0, 2^n).

use casq_sdk::{Circuit, Engine, RunOptions};
use casq_tutorial::connect;

/// Draw one n-bit random integer from a single measurement of n qubits.
async fn quantum_random(client: &casq_sdk::Client, n: usize) -> casq_sdk::Result<u64> {
    let mut circuit = Circuit::new(n);
    for q in 0..n {
        circuit.h(q);
    }
    let result = client
        .run(&circuit, RunOptions::new().engine(Engine::Statevector).shots(1))
        .await?;
    // A single shot yields exactly one measured bitstring in the counts map.
    let bitstring = result
        .counts()
        .keys()
        .next()
        .cloned()
        .unwrap_or_else(|| "0".repeat(n));
    Ok(u64::from_str_radix(&bitstring, 2).unwrap_or(0))
}

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;

    println!("Ten quantum-random bytes (0..255):");
    for _ in 0..10 {
        let value = quantum_random(&client, 8).await?;
        print!("{value:>4}");
    }
    println!();

    // Roll a quantum die: draw 3 bits, keep values 1..=6 (reject 0 and 7).
    println!("\nRolling a fair quantum die five times:");
    let mut rolls = 0;
    while rolls < 5 {
        let v = quantum_random(&client, 3).await?;
        if (1..=6).contains(&v) {
            print!(" {v}");
            rolls += 1;
        }
    }
    println!();
    Ok(())
}
