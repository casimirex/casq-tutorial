//! Lesson 23 — Transpilation: fitting a circuit to real hardware.
//!
//! A real device runs only a fixed set of *native* gates. Your circuit's gates
//! (H, Toffoli, ...) must be rewritten — *transpiled* — into that basis before
//! they can run. The rewrite is exact (same computation) but almost always uses
//! *more* gates, which is why deeper circuits are harder on noisy hardware.
//!
//! We transpile a Bell circuit and a Toffoli to the native basis {rz, ry, cx}
//! and watch the gate count grow.

use casq_sdk::Circuit;
use casq_tutorial::{connect, print_histogram};

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;

    // Bell: one non-native gate (H) plus a native CX.
    let mut bell = Circuit::new(2);
    bell.h(0).cx(0, 1);
    let t = client.transpile(&bell).await?;
    println!("Bell state:");
    println!("  {} gates -> {} native gates {:?}", t.original_gate_count, t.transpiled_gate_count, t.basis);
    println!("  native ops: {:?}", t.operations.iter().map(|o| o.gate.as_str()).collect::<Vec<_>>());
    println!("  fully native: {}", t.fully_native);

    // The transpiled circuit is still a Bell state — run it to prove it.
    let native = t.to_circuit(2);
    let run = client.run(&native, casq_sdk::RunOptions::new().shots(2000)).await?;
    println!("  measured (still |00>/|11> only):");
    print_histogram(run.counts());

    // Toffoli: a single 3-qubit gate explodes into many native gates.
    let mut toffoli = Circuit::new(3);
    toffoli.x(0).x(1).ccx(0, 1, 2);
    let t = client.transpile(&toffoli).await?;
    use std::collections::BTreeMap;
    let mut histogram: BTreeMap<&str, usize> = BTreeMap::new();
    for op in &t.operations {
        *histogram.entry(op.gate.as_str()).or_default() += 1;
    }
    println!("\nToffoli circuit:");
    println!("  {} gates -> {} native gates {:?}", t.original_gate_count, t.transpiled_gate_count, histogram);

    println!("\nThe rewrite is exact but costs gates — one Toffoli becomes ~20 native");
    println!("operations. On noisy hardware, that gate-count blow-up is the enemy.");
    Ok(())
}
