//! Lesson 25 — The Circuit Library: building blocks, and controlled rotations.
//!
//! Every algorithm in this course — Deutsch–Jozsa, Grover, Shor, VQE — is a
//! clever arrangement of a small set of *building-block* circuits. CasimirQ
//! ships 53 of them as a browsable **Circuit Library** (its own section in the
//! web app; each one is a real saved circuit you can open in the builder).
//!
//! Lessons 3 and 6 toured the single- and two-qubit gates. This lesson fills in
//! the pieces we hadn't driven yet — the **controlled rotations** `crx/cry/crz`
//! (new in `casq-sdk`), controlled-Hadamard, and a reusable *subroutine* (the
//! swap test) — so the SDK's fluent API can express the whole Library.
//!
//! Bit-order note: measurement strings print with **qubit 0 on the right**, so
//! `|11>` means q0=1, q1=1.

use casq_sdk::{Circuit, RunOptions};
use casq_tutorial::{connect, print_histogram};
use std::f64::consts::PI;

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;

    macro_rules! show {
        ($title:expr, $circuit:expr) => {{
            let result = client.run(&$circuit, RunOptions::new().shots(1000)).await?;
            println!("{}", $title);
            print_histogram(result.counts());
            println!();
        }};
    }

    println!("== Controlled rotations: crx / cry / crz ==\n");

    // --- Controlled-Ry: rotate the target *only* when the control is set ---
    // Ry(pi) maps |0> -> |1>. So with the control q0=1, cry drives q1 to |1>.
    let mut cry_on = Circuit::new(2);
    cry_on.x(0).cry(0, 1, PI);
    show!("CRY(pi)  |01> -> |11>   (control q0=1 -> Ry(pi) flips q1):", cry_on);

    // Control q0=0: the rotation is skipped, the target is untouched.
    let mut cry_off = Circuit::new(2);
    cry_off.cry(0, 1, PI);
    show!("CRY(pi)  |00> -> |00>   (control q0=0 -> no rotation):", cry_off);

    // --- Controlled-Rz: a *phase* rotation, made visible by interference ---
    // Rz only adds a phase you can't read directly. Put the target in |+>, apply
    // a controlled Rz(pi) (= a controlled-Z up to phase), then H: the hidden
    // phase becomes a bit flip on q1. With the control set, q1 reads 1.
    let mut crz_on = Circuit::new(2);
    crz_on.x(0).h(1).crz(0, 1, PI).h(1);
    show!("CRZ(pi)  kickback with control q0=1 -> q1 reads 1  (|11>):", crz_on);

    // Control q0=0: no phase, so q1 returns to 0.
    let mut crz_off = Circuit::new(2);
    crz_off.h(1).crz(0, 1, PI).h(1);
    show!("CRZ(pi)  no control (q0=0) -> q1 reads 0  (|00>):", crz_off);

    println!("== More building blocks ==\n");

    // --- Controlled-Hadamard: put the target into superposition on demand ---
    // With q0=1, ch applies H to q1, so q1 becomes 50/50 (|01> and |11>).
    let mut ch = Circuit::new(2);
    ch.x(0).ch(0, 1);
    show!("CH       control q0=1 -> H on q1 -> 50/50 over |01>,|11>:", ch);

    // --- A subroutine: the swap test (are two states equal?) ---
    // Ancilla q0 controls a swap of q1 and q2, sandwiched by Hadamards. If the
    // two states are identical the interference is perfect and the ancilla
    // always reads 0. Here q1 = q2 = |0>, so the whole register reads |000> and
    // the ancilla (q0, the rightmost bit) is deterministically 0.
    let mut swap_test = Circuit::new(3);
    swap_test.h(0).cswap(0, 1, 2).h(0);
    show!("SWAP TEST  identical inputs -> |000|, ancilla q0 = 0 (states match):", swap_test);

    println!("These join the gates from Lessons 3 and 6 to cover every gate in the");
    println!("Circuit Library. Browse all 53 building blocks in the web app under");
    println!("\"Circuit Library\" — open any of them straight into the builder, or");
    println!("rebuild them here with the same fluent API you just used.");
    Ok(())
}
