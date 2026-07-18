//! Lesson 10 — The Quantum Fourier Transform.
//!
//! The QFT is the quantum analogue of the discrete Fourier transform and the
//! engine behind phase estimation and Shor's algorithm. It maps a computational
//! basis state into a superposition whose *phases* encode the input — using only
//! O(n^2) gates versus O(n·2^n) for the classical FFT over the same amplitudes.
//!
//! Here we call the algorithms API to build and run the QFT and report the
//! circuit it produced.

use casq_tutorial::connect;

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let algos = client.algorithms();

    println!("QFT circuit cost grows quadratically with qubit count:\n");
    println!("  n   gates   depth   state size");
    for n in 2..=6 {
        let r = algos.qft(n).await?;
        println!(
            "  {n}   {:>5}   {:>5}   {:>10}",
            r.gate_count, r.depth, r.state_size
        );
    }

    println!("\nNotice the gate count scales like ~n^2 (n Hadamards plus the");
    println!("controlled-phase 'staircase'), the heart of its exponential edge.");
    Ok(())
}
