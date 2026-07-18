//! Lesson 19 — BB84 quantum key distribution.
//!
//! BB84 lets two parties agree on a secret key whose security rests on physics,
//! not on computational hardness. The key idea: measuring a qubit in the *wrong*
//! basis randomizes the result and disturbs the state — so an eavesdropper
//! ("Eve") inevitably leaves detectable errors.
//!
//! Protocol:
//!   1. Alice sends each qubit prepared with a random bit in a random basis (Z or X).
//!   2. Bob measures each qubit in his own random basis.
//!   3. They publicly compare *bases* (not bits) and keep only the positions
//!      where the bases matched — the "sifted key".
//!   4. They sacrifice a few sifted bits to estimate the error rate. High error
//!      => someone was listening; abort.
//!
//! We build every qubit's preparation and measurement as a raw circuit, and use
//! superposition itself as the source of all the random choices.

use casq_sdk::{Circuit, RunOptions};
use casq_tutorial::{connect, qubit_bit};

/// Draw `n` physically-random bits from superposition, measured once.
///
/// We draw in small chunks: a wide all-Hadamard circuit is a *Clifford* circuit,
/// which the auto-selected stabilizer engine only samples correctly up to a
/// modest qubit count. Small chunks keep every draw well inside that limit.
async fn draw_bits(client: &casq_sdk::Client, n: usize) -> casq_sdk::Result<Vec<u8>> {
    const CHUNK: usize = 8;
    let mut bits = Vec::with_capacity(n);
    let mut remaining = n;
    while remaining > 0 {
        let k = remaining.min(CHUNK);
        let mut c = Circuit::new(k);
        for q in 0..k {
            c.h(q);
        }
        let r = client.run(&c, RunOptions::new().shots(1)).await?;
        let s = r.counts().keys().next().cloned().unwrap_or_default();
        for q in 0..k {
            bits.push(if qubit_bit(&s, q) == Some('1') { 1 } else { 0 });
        }
        remaining -= k;
    }
    Ok(bits)
}

/// Measure a single-qubit circuit once and return the resulting bit.
async fn measure_one(client: &casq_sdk::Client, c: &Circuit) -> casq_sdk::Result<u8> {
    let r = client.run(c, RunOptions::new().shots(1)).await?;
    let s = r.counts().keys().next().cloned().unwrap_or_default();
    Ok(if s == "1" { 1 } else { 0 })
}

/// Prepare a qubit for Alice: bit b in basis (0 = Z, 1 = X).
fn prepare(bit: u8, basis: u8) -> Circuit {
    let mut c = Circuit::new(1);
    if bit == 1 {
        c.x(0);
    }
    if basis == 1 {
        c.h(0); // rotate into the X basis (|0>->|+>, |1>->|->)
    }
    c
}

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let n = 20;

    // All random choices come from quantum measurements.
    let alice_bits = draw_bits(&client, n).await?;
    let alice_bases = draw_bits(&client, n).await?;
    let bob_bases = draw_bits(&client, n).await?;

    // Positions where Alice and Bob happened to pick the same basis: the sift.
    let sift: Vec<usize> = (0..n).filter(|&i| alice_bases[i] == bob_bases[i]).collect();

    // --- Case 1: no eavesdropper. Bob measures Alice's qubits directly. ---
    let mut bob_bits = vec![0u8; n];
    for i in 0..n {
        let mut c = prepare(alice_bits[i], alice_bases[i]);
        if bob_bases[i] == 1 {
            c.h(0); // Bob's measurement basis
        }
        bob_bits[i] = measure_one(&client, &c).await?;
    }
    let errors_clean = sift.iter().filter(|&&i| bob_bits[i] != alice_bits[i]).count();

    let alice_key: String = sift.iter().map(|&i| char::from(b'0' + alice_bits[i])).collect();
    let bob_key: String = sift.iter().map(|&i| char::from(b'0' + bob_bits[i])).collect();
    println!("=== No eavesdropper ===");
    println!("  sifted {} of {} qubits", sift.len(), n);
    println!("  Alice key: {alice_key}");
    println!("  Bob   key: {bob_key}");
    println!(
        "  error rate on sifted key: {}/{} = {:.1}%  (keys match: {})\n",
        errors_clean,
        sift.len(),
        100.0 * errors_clean as f64 / sift.len().max(1) as f64,
        alice_key == bob_key
    );

    // --- Case 2: Eve intercepts, measures in a random basis, and re-sends. ---
    let eve_bases = draw_bits(&client, n).await?;
    let mut errors_eve = 0;
    for &i in &sift {
        // Eve measures Alice's qubit in her basis (this collapses it).
        let mut e = prepare(alice_bits[i], alice_bases[i]);
        if eve_bases[i] == 1 {
            e.h(0);
        }
        let eve_bit = measure_one(&client, &e).await?;

        // Eve re-prepares what she saw, in her basis, and forwards it to Bob.
        let mut b = prepare(eve_bit, eve_bases[i]);
        if bob_bases[i] == 1 {
            b.h(0);
        }
        let bob_bit = measure_one(&client, &b).await?;
        if bob_bit != alice_bits[i] {
            errors_eve += 1;
        }
    }
    println!("=== With an eavesdropper ===");
    println!(
        "  error rate on sifted key: {}/{} = {:.1}%",
        errors_eve,
        sift.len(),
        100.0 * errors_eve as f64 / sift.len().max(1) as f64
    );
    println!("  Eve's meddling injects ~25% errors on the sifted key. Alice and");
    println!("  Bob see the spike, conclude the channel is tapped, and discard it.");
    Ok(())
}
