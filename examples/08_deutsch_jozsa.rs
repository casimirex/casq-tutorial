//! Lesson 8 — Deutsch-Jozsa: a first taste of quantum advantage.
//!
//! Given a black-box function f on n input bits that is promised to be either
//! *constant* (same output for every input) or *balanced* (0 for half the
//! inputs, 1 for the other half), classically you might need up to 2^(n-1)+1
//! queries to be sure. Deutsch-Jozsa decides it with a single evaluation of f.
//!
//! We build the oracle ourselves as a circuit and read the answer off the input
//! register: measure all zeros => constant, anything else => balanced.

use casq_sdk::{Circuit, Engine, RunOptions};
use casq_tutorial::{connect, qubit_bit};

/// The kind of oracle to build.
#[derive(Clone, Copy)]
enum Oracle {
    /// f(x) = 0 for all x.
    ConstantZero,
    /// f(x) = 1 for all x.
    ConstantOne,
    /// f(x) = x0 XOR x1 XOR ... (parity) — a balanced function.
    Balanced,
}

/// Build the full Deutsch-Jozsa circuit for `n` input qubits (+1 ancilla).
fn deutsch_jozsa(n: usize, oracle: Oracle) -> Circuit {
    let ancilla = n; // qubits 0..n are inputs; qubit n is the ancilla
    let mut c = Circuit::new(n + 1);

    // Prepare the ancilla in |-> and the inputs in uniform superposition.
    c.x(ancilla).h(ancilla);
    for q in 0..n {
        c.h(q);
    }

    // Apply the oracle U_f: |x>|y> -> |x>|y XOR f(x)>.
    match oracle {
        Oracle::ConstantZero => { /* identity: f is always 0 */ }
        Oracle::ConstantOne => {
            c.x(ancilla); // flip y for every x
        }
        Oracle::Balanced => {
            for q in 0..n {
                c.cx(q, ancilla); // y XOR (x0 XOR x1 XOR ...)
            }
        }
    }

    // Interfere the input register back and measure it.
    for q in 0..n {
        c.h(q);
    }
    c
}

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let n = 3;

    for (label, oracle) in [
        ("constant (always 0)", Oracle::ConstantZero),
        ("constant (always 1)", Oracle::ConstantOne),
        ("balanced (parity)", Oracle::Balanced),
    ] {
        let circuit = deutsch_jozsa(n, oracle);
        let result = client
            .run(&circuit, RunOptions::new().engine(Engine::Statevector).shots(1))
            .await?;

        // Look only at the n input qubits of the single measured bitstring.
        let bitstring = result.counts().keys().next().cloned().unwrap_or_default();
        let inputs_all_zero = (0..n).all(|q| qubit_bit(&bitstring, q) == Some('0'));
        let verdict = if inputs_all_zero { "CONSTANT" } else { "BALANCED" };

        println!("oracle: {label:<22} -> input register {} => {verdict}", &bitstring);
    }

    println!("\nOne query each — no classical algorithm can do that with certainty.");
    Ok(())
}
