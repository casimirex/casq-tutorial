//! Lesson 17 — Quantum error correction.
//!
//! Qubits are fragile: they decohere and gates are imperfect. Quantum error
//! correction (QEC) protects *logical* qubits by spreading them across many
//! *physical* qubits and repeatedly measuring "stabilizers" — checks that reveal
//! errors without disturbing the encoded information.
//!
//! Two landmark codes:
//!   - Steane: 7 physical qubits per logical qubit, distance 3.
//!   - Shor:   9 physical qubits per logical qubit, distance 3.
//! A distance-3 code can correct any single-qubit error.

use casq_tutorial::connect;

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let adv = client.advanced();

    println!("Available error-correcting codes:\n");
    for code in adv.qec_codes().await? {
        let correct = (code.distance - 1) / 2;
        println!(
            "  {:<7} {} physical -> {} logical, distance {} (corrects up to {} error(s))",
            code.id, code.n_physical, code.n_logical, code.distance, correct
        );
        println!("           {}", code.error_correction_capability);
    }

    // Encode the logical |0> with the Steane code and read its syndrome.
    println!("\nEncoding logical |0> with the Steane code:");
    let encoded = adv.encode("steane", Some(&[0])).await?;
    println!(
        "  spread across {} physical qubits; freshly-encoded syndrome: {:?}",
        encoded.n_physical, encoded.syndrome
    );

    let syn = adv.syndrome("steane", Some(&[0])).await?;
    let clean = syn.syndrome.iter().all(|&s| s == 0);
    println!(
        "  measured syndrome {:?} -> {}",
        syn.syndrome,
        if clean { "all zero: no error detected" } else { "non-zero: an error was flagged" }
    );

    println!("\nAn all-zero syndrome means the code block is error-free. A non-zero");
    println!("pattern would point the decoder at which qubit to correct — without");
    println!("ever measuring (and thus destroying) the logical state itself.");
    Ok(())
}
