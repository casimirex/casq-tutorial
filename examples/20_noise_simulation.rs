//! Lesson 20 — Simulating noise with the density-matrix engine.
//!
//! Lesson 18 described noise *channels*; this lesson actually *runs* circuits
//! under them. The density-matrix engine evolves the full state ρ, so it can
//! represent the mixed states that noise produces — and report how far from
//! ideal the result is via **purity** (Tr(ρ²), 1 = pure) and **fidelity**
//! (overlap with the noiseless state).

use casq_sdk::advanced::{NoiseChannelConfig, NoiseSimOptions};
use casq_sdk::Circuit;
use casq_tutorial::{connect, print_histogram};

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let adv = client.advanced();

    // A Bell circuit — perfectly correlated when noiseless.
    let mut bell = Circuit::new(2);
    bell.h(0).cx(0, 1);

    // 1. Noiseless baseline.
    let clean = adv
        .simulate_noise(&bell, &[], NoiseSimOptions { compute_fidelity: true, ..Default::default() })
        .await?;
    println!("Noiseless Bell:  purity {:.3}  fidelity {:.3}", clean.purity, clean.fidelity.unwrap());

    // 2. Sweep depolarizing strength and watch the state degrade.
    println!("\nDepolarizing noise after every gate:");
    println!("   p      purity   fidelity");
    for p in [0.0, 0.05, 0.1, 0.2, 0.4] {
        let r = adv
            .simulate_noise(
                &bell,
                &[NoiseChannelConfig::depolarizing(p)],
                NoiseSimOptions { compute_fidelity: true, ..Default::default() },
            )
            .await?;
        println!("  {p:.2}    {:.3}    {:.3}", r.purity, r.fidelity.unwrap());
    }

    // 3. See the error states appear in the sampled counts.
    let noisy = adv
        .simulate_noise(
            &bell,
            &[NoiseChannelConfig::depolarizing(0.15)],
            NoiseSimOptions { shots: Some(2000), ..Default::default() },
        )
        .await?;
    println!("\nBell under depolarizing p=0.15 ({} shots) — note the 01/10 leakage:", 2000);
    print_histogram(&noisy.counts);

    // 4. Amplitude damping: |1> relaxes toward |0> by exactly gamma.
    let mut one = Circuit::new(1);
    one.x(0);
    println!("\nAmplitude damping on |1> (P(0) should equal gamma):");
    for gamma in [0.0, 0.25, 0.5, 1.0] {
        let r = adv
            .simulate_noise(&one, &[NoiseChannelConfig::amplitude_damping(gamma)], NoiseSimOptions::default())
            .await?;
        println!("  gamma {gamma:.2} -> P(0) = {:.3}", r.probabilities.get("0").copied().unwrap_or(0.0));
    }

    Ok(())
}
