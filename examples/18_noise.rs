//! Lesson 18 — Noise and the NISQ reality.
//!
//! Today's machines are "NISQ" — Noisy Intermediate-Scale Quantum. Every gate,
//! idle moment, and measurement can introduce errors. Understanding the noise
//! *channels* (how errors act) and a device's *characteristics* (how good its
//! qubits are) is essential to getting useful results before full error
//! correction arrives.
//!
//! Key channels: depolarizing (random Pauli error), amplitude damping (energy
//! loss, T1), phase damping (dephasing, T2), and bit/phase flips.

use casq_sdk::advanced::NoiseChannel;
use casq_tutorial::connect;

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let adv = client.advanced();

    let catalog = adv.noise_catalog().await?;
    println!("Supported noise channels: {:?}", catalog.channels);
    println!("Built-in device models:   {:?}\n", catalog.models);

    // Validate a small noise specification.
    let channels = vec![
        NoiseChannel::new("depolarizing", ("probability", 0.01), 0),
        NoiseChannel::new("amplitude_damping", ("gamma", 0.02), 1),
    ];
    let validation = adv.validate_noise(&channels).await?;
    println!("Noise spec valid: {}", validation.all_valid);

    // Characterize each built-in device model.
    println!("\nDevice characteristics:");
    for model in &catalog.models {
        let dev = adv.characterize(model).await?;
        println!(
            "  {:<14} qubits: {}",
            model,
            dev.n_qubits().map_or("n/a".to_string(), |n| n.to_string())
        );
    }

    println!("\nWhy it matters: circuit depth is limited by how long qubits stay");
    println!("coherent (T1/T2). Shallow, noise-aware circuits — and the variational");
    println!("algorithms from earlier lessons — are the pragmatic response.");
    Ok(())
}
