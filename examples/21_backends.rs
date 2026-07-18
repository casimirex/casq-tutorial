//! Lesson 21 — Choosing a backend.
//!
//! A circuit doesn't have to run on the default simulator. casimirQ exposes
//! *backends* — simulators, an emulated device, and (when configured) a real
//! QPU — behind one interface. Picking where a circuit runs is just a backend
//! id; the request is otherwise identical.
//!
//! We list the backends, then run the same Bell circuit on the exact local
//! simulator and on the emulated QPU to see how device noise and a restricted
//! native gate set change the result.

use casq_sdk::backends::BackendRunOptions;
use casq_sdk::Circuit;
use casq_tutorial::{connect, print_histogram};

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let backends = client.backends();

    println!("Available backends:");
    for b in backends.list().await? {
        let c = &b.capabilities;
        println!(
            "  {:16} {:18} available={:<5} maxQ={:<2} noise={:<5} conn={}",
            b.id, b.backend_type, b.available, c.max_qubits, c.supports_noise, c.connectivity
        );
    }

    // The same Bell circuit, run on two different backends.
    let mut bell = Circuit::new(2);
    bell.h(0).cx(0, 1);
    let opts = || BackendRunOptions { shots: Some(2000), ..Default::default() };

    println!("\n--- local-simulator (exact) ---");
    let local = backends.run("local-simulator", &bell, opts()).await?;
    print_histogram(&local.counts);
    println!(
        "purity {:?}, native-gate fraction {:?}",
        local.purity(),
        local.native_gate_fraction()
    );

    println!("\n--- emulated-qpu (device noise + restricted native set) ---");
    let emulated = backends.run("emulated-qpu", &bell, opts()).await?;
    print_histogram(&emulated.counts);
    println!(
        "purity {:.3}, native-gate fraction {:?}",
        emulated.purity().unwrap_or(1.0),
        emulated.native_gate_fraction()
    );

    println!(
        "\nSame circuit, different targets: the emulated device shows noise (purity < 1)\n\
         and a native fraction of 0.5 — the Hadamard isn't in its native basis, so it\n\
         would be transpiled before running on real hardware."
    );
    Ok(())
}
