//! Lesson 12 — Quantum teleportation.
//!
//! Teleportation moves an unknown qubit state from a sender to a receiver using
//! a shared entangled pair plus two classical bits — without ever copying the
//! state (the no-cloning theorem forbids that). The original is destroyed by
//! the sender's measurement; the receiver reconstructs it with corrections.
//!
//! casq-sdk exposes it through the algorithms API: give the amplitudes of the
//! state to send, and it reports the receiver's probabilities and the fidelity
//! between what was sent and what arrived.

use casq_tutorial::connect;

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let algos = client.algorithms();

    // A few normalized single-qubit states to teleport: (alpha, beta).
    let states = [
        ("|0>", 1.0, 0.0),
        ("|1>", 0.0, 1.0),
        ("|+>", std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2),
        ("0.6|0>+0.8|1>", 0.6, 0.8),
    ];

    println!("Teleporting single-qubit states:\n");
    println!("  state             P(0)    P(1)    fidelity   verified");
    for (label, alpha, beta) in states {
        let r = algos.teleport(alpha, beta).await?;
        println!(
            "  {label:<16}  {:.3}   {:.3}   {:.4}     {}",
            r.teleported_probabilities.prob0,
            r.teleported_probabilities.prob1,
            r.fidelity,
            r.verified
        );
    }

    println!("\nFidelity ~1.0 means the received state matches the one sent.");
    Ok(())
}
