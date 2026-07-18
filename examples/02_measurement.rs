//! Lesson 2 — Measurement, shots, and probabilities.
//!
//! Quantum results are statistical. One run gives a single outcome; to see the
//! underlying distribution you sample many "shots". This lesson contrasts the
//! exact probabilities (from the statevector) with the sampled counts, and
//! shows how the sample converges as the shot count grows.

use casq_sdk::{Circuit, Engine, RunOptions};
use casq_tutorial::{connect, print_histogram};

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;

    // A biased single qubit: a small rotation tilts the |0>/|1> odds.
    // Ry(theta) gives P(1) = sin^2(theta/2). theta = pi/3 -> P(1) = 0.25.
    let theta = std::f64::consts::FRAC_PI_3;
    let mut circuit = Circuit::new(1);
    circuit.ry(0, theta);

    // Exact probabilities: ask for the statevector once.
    let exact = client
        .run(&circuit, RunOptions::new().engine(Engine::Statevector).shots(1))
        .await?;
    println!("Exact probabilities (theory: P(1) = sin^2(theta/2) = 0.25):");
    for (state, p) in exact.probabilities() {
        println!("  P(|{state}>) = {p:.4}");
    }

    // Sampling converges to the exact distribution as shots increase.
    for shots in [10, 100, 1000, 10_000] {
        let run = client
            .run(&circuit, RunOptions::new().shots(shots))
            .await?;
        let ones = run.counts().get("1").copied().unwrap_or(0);
        println!(
            "\n{shots:>6} shots -> P(1) ~= {:.4}",
            ones as f64 / shots as f64
        );
        print_histogram(run.counts());
    }
    Ok(())
}
