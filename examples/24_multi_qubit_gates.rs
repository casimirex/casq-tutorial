//! Lesson 24 — Multi-qubit gates: CNOT, CZ, SWAP, Toffoli, Fredkin.
//!
//! Single-qubit gates (Lesson 3) rotate one qubit. The gates here *couple*
//! qubits: they let one qubit's value control what happens to another. That
//! coupling is where entanglement — and quantum logic — comes from. We drive
//! each gate with a definite input and read the definite output, so the effect
//! is unmistakable.
//!
//! Bit-order note: measurement strings are printed with **qubit 0 on the right**
//! (so `|011>` means q0=1, q1=1, q2=0). Every arrow below is written the same
//! way, matching the histogram underneath it.

use casq_sdk::{Circuit, RunOptions};
use casq_tutorial::{connect, print_histogram};

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;

    // Helper: run a circuit and show its histogram. (Every circuit here is
    // deterministic, so a single bar carries all the shots.)
    macro_rules! show {
        ($title:expr, $circuit:expr) => {{
            let result = client.run(&$circuit, RunOptions::new().shots(1000)).await?;
            println!("{}", $title);
            print_histogram(result.counts());
            println!();
        }};
    }

    // --- CNOT (CX): flip the target iff the control is |1> ---
    // cx(0, 1): control is q0 (the right bit), target is q1. Set q0=1, so the
    // target flips: |001>... here just two qubits, |01> -> |11>.
    let mut cnot = Circuit::new(2);
    cnot.x(0).cx(0, 1);
    show!("CNOT  |01> -> |11>   (control q0=1 flips target q1):", cnot);

    // Control q0=0: the target is left alone.
    let mut cnot0 = Circuit::new(2);
    cnot0.cx(0, 1);
    show!("CNOT  |00> -> |00>   (control q0=0, nothing happens):", cnot0);

    // --- CZ: a *phase* flip on |11>, made visible by interference ---
    // CZ is diagonal, so on a basis state it only adds a phase you cannot
    // measure directly. Put the control (q0) in |+> and the target (q1) in |1>:
    // CZ then acts as a Z on q0 (phase kickback), turning |+> into |->. A final
    // H maps |-> to |1>, so q0 reads 1 — a measurable consequence of a hidden
    // phase. (q1 stays 1, so the string is |11>.)
    let mut cz_kick = Circuit::new(2);
    cz_kick.h(0).x(1).cz(0, 1).h(0);
    show!("CZ    kickback: H·(CZ, target q1=1)·H -> q0 reads 1  (|11>):", cz_kick);

    // Target q1=0: no kickback, so q0 returns to 0.
    let mut cz_none = Circuit::new(2);
    cz_none.h(0).cz(0, 1).h(0);
    show!("CZ    no kickback (target q1=0) -> q0 reads 0  (|00>):", cz_none);

    // --- SWAP: exchange two qubits ---
    // Set q0=1, q1=0, then swap: the values trade places.
    let mut swap = Circuit::new(2);
    swap.x(0).swap(0, 1);
    show!("SWAP  |01> -> |10>   (q0 and q1 trade values):", swap);

    // --- Toffoli (CCNOT): flip the target iff BOTH controls are |1> ---
    // A reversible AND: q2 becomes q2 XOR (q0 AND q1).
    let mut toffoli_both = Circuit::new(3);
    toffoli_both.x(0).x(1).ccx(0, 1, 2);
    show!("CCX   |011> -> |111>  (both controls 1 -> flip target q2):", toffoli_both);

    let mut toffoli_one = Circuit::new(3);
    toffoli_one.x(0).ccx(0, 1, 2);
    show!("CCX   |001> -> |001>  (only one control -> no flip):", toffoli_one);

    // --- Fredkin (CSWAP): swap two targets iff the control is |1> ---
    // Control q0=1, targets q1 and q2 swap.
    let mut fredkin_on = Circuit::new(3);
    fredkin_on.x(0).x(1).cswap(0, 1, 2);
    show!("CSWAP |011> -> |101>  (control q0=1 -> swap q1 and q2):", fredkin_on);

    // Control q0=0: the targets are left as they are.
    let mut fredkin_off = Circuit::new(3);
    fredkin_off.x(1).cswap(0, 1, 2);
    show!("CSWAP |010> -> |010>  (control q0=0 -> no swap):", fredkin_off);

    println!("CNOT and CZ are the two-qubit workhorses; SWAP moves data; Toffoli and");
    println!("Fredkin are reversible AND / controlled-swap — enough to build *any*");
    println!("classical logic, reversibly. Lesson 23 showed how hardware decomposes");
    println!("all of them into its native gate set.");
    Ok(())
}
